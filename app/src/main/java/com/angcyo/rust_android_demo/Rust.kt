package com.angcyo.rust_android_demo

import android.graphics.Path
import androidx.annotation.Keep

@Keep
object Rust {
    init {
        System.loadLibrary("rust")
    }

    //region ----测试方法----

    external fun test(): Float

    external fun testFile(path: String): String

    external fun testPath(path: Path): Path?

    external fun writeFile(content: String, append: Boolean, path: String): Boolean

    external fun readFile(path: String): String

    //endregion ----测试方法----
}