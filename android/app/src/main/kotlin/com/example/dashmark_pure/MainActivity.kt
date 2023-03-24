package com.example.dashmark_pure

import android.app.Fragment
import android.graphics.SurfaceTexture
import android.os.Bundle
import android.view.Surface
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import io.flutter.view.TextureRegistry

class MainActivity: FlutterActivity() {
    companion object {
        init {
            System.loadLibrary("native")
        }
    }

    private lateinit var channel: MethodChannel;
    private lateinit var surface: TextureRegistry.SurfaceTextureEntry;
    private lateinit var surfaceTexture: SurfaceTexture;

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        print("Arrived @ configureFlutterEngine!")

        channel = MethodChannel(flutterEngine.dartExecutor.binaryMessenger, "sturdykeep.com/channel")
        channel.setMethodCallHandler { call, result ->
            if (call.method == "initTexture") {
                surface = flutterEngine.renderer.createSurfaceTexture()
                val id = surface.id()
                surfaceTexture = surface.surfaceTexture()

                val rustBridge = RustBridge()
                rustBridge.initTexture(surfaceTexture)

                result.success(id)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }
}

class RustBridge {
    companion object {
        @JvmStatic
        private external fun nativeInitTexture(surfaceTexture: SurfaceTexture)
    }

    fun initTexture(surfaceTexture: SurfaceTexture) {
        nativeInitTexture(surfaceTexture)
    }
}