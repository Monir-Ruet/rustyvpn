package dev.dioxus.main.jni

import android.content.Context
import android.content.Intent
import android.net.VpnService
import android.util.Log
import dev.dioxus.main.service.VpnHandler

object VpnHelper {
    @JvmStatic
    fun startVpn(context: Context) {
        Log.d("VpnHelper", "Requesting VPN permission")
        val intent = VpnService.prepare(context)
        if (intent != null) {
            // Need VPN permission
            if (context is android.app.Activity) {
                context.startActivityForResult(intent, 1001)
            } else {
                Log.w("VpnHelper", "Context is not Activity — cannot request VPN permission interactively")
            }
        } else {
            Log.d("VpnHelper", "VPN permission already granted")
            startVpnService(context)
        }
    }

    @JvmStatic
    fun startVpnService(context: Context) {
        Log.d("VpnHelper", "Starting VPN service")
        val intent = Intent(context, VpnHandler::class.java).apply {
            action = VpnHandler.ACTION_CONNECT
        }
        context.startService(intent)
    }

    @JvmStatic
    fun stopVpnService(context: Context) {
        Log.d("VpnHelper", "Stopping VPN service")
        val intent = Intent(context, VpnHandler::class.java).apply {
            action = VpnHandler.ACTION_DISCONNECT
        }
        context.startService(intent)
    }
}