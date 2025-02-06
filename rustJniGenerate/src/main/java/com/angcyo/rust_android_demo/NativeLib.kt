package com.angcyo.rust_android_demo

class NativeLib {

    /**
     * A native method that is implemented by the 'rust_android_demo' native library,
     * which is packaged with this application.
     */
    external fun stringFromJNI(): String

    companion object {
        // Used to load the 'rust_android_demo' library on application startup.
        init {
            System.loadLibrary("rust_android_demo")
        }
    }
}