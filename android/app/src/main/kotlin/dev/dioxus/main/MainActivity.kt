package dev.dioxus.main;

// need to re-export buildconfig down from the parent
import android.os.Bundle
import com.orionexx.rustyvpn.BuildConfig;
import dev.dioxus.main.jni.NativeBridge

typealias BuildConfig = BuildConfig;


class MainActivity : WryActivity(){
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        NativeBridge.initBridge(this.classLoader);
    }
}
