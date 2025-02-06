package com.angcyo.rust_android_demo

import android.graphics.Path
import android.os.Bundle
import android.view.View
import android.widget.TextView
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContentView(R.layout.activity_main)
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main)) { v, insets ->
            val systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars())
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
            insets
        }

        //--init
        Rust

        //--test
        val cacheFilePath = "${externalCacheDir?.absolutePath}/test.txt"
        val resultView = findViewById<TextView>(R.id.result_view)
        findViewById<View>(R.id.test)?.setOnClickListener {
            resultView.text = "rust result->${Rust.test()}"
        }
        findViewById<View>(R.id.test_file)?.setOnClickListener {
            resultView.text = "rust result->${cacheFilePath}:\n${Rust.testFile(cacheFilePath)}"
        }
        findViewById<View>(R.id.test_path)?.setOnClickListener {
            resultView.text = "rust result->${Rust.testPath(Path())}"
        }
        findViewById<View>(R.id.test_write)?.setOnClickListener {
            resultView.text =
                "rust result->${
                    Rust.writeFile(
                        "${System.currentTimeMillis()}\n",
                        true,
                        cacheFilePath
                    )
                }"
        }
        findViewById<View>(R.id.test_read)?.setOnClickListener {
            resultView.text = "rust result->${Rust.readFile(cacheFilePath)}"
        }
    }
}