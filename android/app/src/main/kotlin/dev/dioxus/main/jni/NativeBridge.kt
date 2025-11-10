package dev.dioxus.main.jni

object NativeBridge {
    init {
        System.loadLibrary("dioxusmain")
    }

    external fun startVpn(fd: Int)
    @JvmStatic external fun initBridge(loader: ClassLoader)
}