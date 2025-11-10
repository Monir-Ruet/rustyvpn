package dev.dioxus.main.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.net.VpnService
import android.os.Build
import android.os.IBinder
import android.os.ParcelFileDescriptor
import android.system.OsConstants
import android.util.Log
import dev.dioxus.main.jni.NativeBridge
import java.io.FileInputStream
import java.io.FileOutputStream
import java.io.IOException
import java.util.concurrent.atomic.AtomicBoolean
import kotlinx.coroutines.*

class VpnHandler : VpnService() {
    companion object {
        private const val TAG = "RustyVPN"
        const val ACTION_CONNECT = "CONNECT"
        const val ACTION_DISCONNECT = "DISCONNECT"
        private const val VPN_ADDRESS = "10.8.0.2"
        private const val VPN_ROUTE = "0.0.0.0"
        private const val NOTIFICATION_ID = 1
        private const val CHANNEL_ID = "vpn_service_channel"
    }

    private var vpnInterface: ParcelFileDescriptor? = null
    private val isRunning = AtomicBoolean(false)

    // Use a service-scoped CoroutineScope so we can cancel everything on destroy
    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private var vpnJob: Job? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.d(TAG, "onStartCommand: ${intent?.action}")

        when (intent?.action) {
            ACTION_CONNECT -> connectVpn()
            ACTION_DISCONNECT -> disconnectVpn()
            else -> {
                // If started without explicit action, do nothing
                Log.d(TAG, "No VPN action provided")
            }
        }
        // If killed, let system recreate the service; we will restore state from intents if needed
        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? {
        // We don't expose a binder
        return null
    }

    private fun connectVpn() {
        if (isRunning.get()) {
            Log.w(TAG, "VPN already running")
            return
        }

        Log.d(TAG, "Starting VPN service")

        // Launch once on service scope
        serviceScope.launch {
            try {
                // Start foreground service FIRST (before establishing VPN) to avoid being killed
                startForegroundServiceSafely()

                // Configure VPN interface
                val builder = Builder().apply {
                    setSession("RustyVPN")
                    setMtu(1500)
                    addAddress(VPN_ADDRESS, 24)
                    // route 0.0.0.0/0
                    addRoute(VPN_ROUTE, 0)
                    addDnsServer("8.8.8.8")
                    addDnsServer("8.8.4.4")
                    // You can add more configuration here (allowed/blocked apps, etc.)
                }

                val established = try {
                    builder.establish()
                } catch (e: Exception) {
                    Log.e(TAG, "Failed to establish VPN interface", e)
                    null
                }

                if (established == null) {
                    Log.e(TAG, "Could not establish VPN interface - aborting connect")
                    stopForeground(true)
                    return@launch
                }

                vpnInterface = established
                isRunning.set(true)
                Log.i(TAG, "VPN interface established successfully")

                val fd = vpnInterface?.fd ?:  return@launch;
                NativeBridge.startVpn(fd)
                
                // Start packet processing loop
                //startPacketProcessing()

            } catch (e: Exception) {
                Log.e(TAG, "VPN connection failed", e)
                disconnectVpn()
            }
        }
    }

    private fun startForegroundServiceSafely() {
        // create channel & base notification
        createNotificationChannel()
        val notification = buildNotification("Starting VPN…")
        // startForeground must be called from the service (we are inside it), this is fine
        try {
            startForeground(NOTIFICATION_ID, notification)
            Log.d(TAG, "Foreground service started")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to call startForeground()", e)
        }
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "VPN Service",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "RustyVPN connection service"
                setShowBadge(false)
                lockscreenVisibility = Notification.VISIBILITY_PUBLIC
            }

            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager?.createNotificationChannel(channel)
        }
    }

    private fun buildNotification(contentText: String = "VPN is connected"): Notification {
        // Build a simple stop action that sends ACTION_DISCONNECT
        val stopIntent = Intent(this, VpnHandler::class.java).apply {
            action = ACTION_DISCONNECT
        }
        val pendingStop = PendingIntent.getService(
            this,
            0,
            stopIntent,
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) PendingIntent.FLAG_IMMUTABLE else 0
        )

        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ID)
                .setContentTitle("RustyVPN")
                .setContentText(contentText)
                .setSmallIcon(android.R.drawable.ic_dialog_info)
                .setOngoing(true)
                .setCategory(Notification.CATEGORY_SERVICE)
                .addAction(android.R.drawable.ic_media_pause, "Disconnect", pendingStop)
                .setVisibility(Notification.VISIBILITY_PUBLIC)
                .build()
        } else {
            @Suppress("DEPRECATION")
            Notification.Builder(this)
                .setContentTitle("RustyVPN")
                .setContentText(contentText)
                .setSmallIcon(android.R.drawable.ic_dialog_info)
                .setOngoing(true)
                .addAction(android.R.drawable.ic_media_pause, "Disconnect", pendingStop)
                .build()
        }
    }

    private fun startPacketProcessing() {
        val iface = vpnInterface
        if (iface == null) {
            Log.e(TAG, "startPacketProcessing called but vpnInterface is null")
            return
        }

        vpnJob = serviceScope.launch {
            Log.d(TAG, "Starting packet processing")

            // wrap streams to the file descriptor
            val input = try {
                FileInputStream(iface.fileDescriptor)
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create FileInputStream", e)
                disconnectVpn()
                return@launch
            }

            val output = try {
                FileOutputStream(iface.fileDescriptor)
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create FileOutputStream", e)
                disconnectVpn()
                return@launch
            }

            val buffer = ByteArray(32767)
            var packetCount = 0

            try {
                while (isRunning.get() && isActive) {
                    try {
                        val length = input.read(buffer)
                        if (length == -1) {
                            // EOF — stop processing
                            Log.d(TAG, "VPN input stream returned EOF (-1)")
                            break
                        }
                        if (length > 0) {
                            packetCount++
                            val packet = buffer.copyOf(length)

                            // Log packet details (non-blocking)
                            logPacketDetails(packet)

                            // Echo packets back (for testing)
                            try {
                                output.write(packet)
                            } catch (ioe: IOException) {
                                Log.e(TAG, "Failed to write packet back to TUN", ioe)
                            }

                            // Update notification occasionally
                            if (packetCount % 10 == 0) {
                                try {
                                    val notif = buildNotification("Packets processed: $packetCount")
                                    val nm = getSystemService(Context.NOTIFICATION_SERVICE) as? NotificationManager
                                    nm?.notify(NOTIFICATION_ID, notif)
                                } catch (e: Exception) {
                                    Log.e(TAG, "Failed updating notification", e)
                                }
                            }
                        } else {
                            // No data read; yield a bit
                            delay(10)
                        }
                    } catch (e: Exception) {
                        if (isRunning.get()) {
                            Log.e(TAG, "Error in packet processing loop", e)
                            delay(100)
                        } else {
                            break
                        }
                    }
                }
            } finally {
                Log.d(TAG, "Packet processing stopped. Total packets: $packetCount")
                try {
                    input.close()
                } catch (ignored: Exception) {}
                try {
                    output.close()
                } catch (ignored: Exception) {}
            }
        }
    }

    private fun logPacketDetails(packet: ByteArray) {
        if (packet.size >= 20) {
            try {
                // first nibble: version
                val version = (packet[0].toInt() and 0xF0) shr 4
                val protocol = packet[9].toInt() and 0xFF

                val protocolName = when (protocol) {
                    OsConstants.IPPROTO_TCP -> "TCP"
                    OsConstants.IPPROTO_UDP -> "UDP"
                    OsConstants.IPPROTO_ICMP -> "ICMP"
                    else -> "UNKNOWN($protocol)"
                }

                Log.d(TAG, "IPv$version $protocolName packet: ${packet.size} bytes")

                // Safe hex dump of first 16 bytes
                val hexDump = packet.take(16).joinToString(" ") {
                    String.format("%02X", it)
                }
                Log.v(TAG, "Packet hex: $hexDump...")

            } catch (e: Exception) {
                Log.e(TAG, "Error parsing packet", e)
            }
        } else {
            Log.d(TAG, "Small packet: ${packet.size} bytes")
        }
    }

    private fun disconnectVpn() {
        Log.d(TAG, "Disconnecting VPN")

        isRunning.set(false)

        // cancel packet processing job
        vpnJob?.cancel()
        vpnJob = null

        // close interface
        try {
            vpnInterface?.close()
        } catch (e: Exception) {
            Log.e(TAG, "Error closing VPN interface", e)
        } finally {
            vpnInterface = null
        }

        // stop foreground and service
        try {
            stopForeground(true)
        } catch (e: Exception) {
            Log.e(TAG, "stopForeground failed", e)
        }

        try {
            stopSelf()
        } catch (e: Exception) {
            Log.e(TAG, "stopSelf failed", e)
        }

        Log.i(TAG, "VPN disconnected")
    }

    override fun onRevoke() {
        Log.d(TAG, "VPN permission revoked")
        disconnectVpn()
        super.onRevoke()
    }

    override fun onDestroy() {
        Log.d(TAG, "VPN service destroyed")
        // cancel everything in the service scope
        try {
            disconnectVpn()
        } catch (e: Exception) {
            Log.e(TAG, "Error while disconnecting in onDestroy", e)
        } finally {
            serviceScope.cancel()
        }
        super.onDestroy()
    }
}
