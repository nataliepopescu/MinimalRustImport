package com.example.minimalrustimport

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val hello_message: TextView = findViewById(R.id.hello_message)
        hello_message.setText("Different Text!")
    }

    // still don't entirely understand why we are adding this here though
    // furthermore, this is the first change I've made to the code and it still compiles
    companion object {
        init {
            System.loadLibrary("simple")
        }
    }
}
