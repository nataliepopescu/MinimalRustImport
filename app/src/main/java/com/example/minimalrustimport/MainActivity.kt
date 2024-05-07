package com.example.minimalrustimport

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import com.example.minimalrustimport.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        binding.helloMessage.text = hello()
        binding.addResult.text = add(6, 6).toString()
    }

    external fun hello(): String?

    external fun add(left: Int, right: Int): Int

    companion object {
        init {
            System.loadLibrary("simple")
        }
    }
}
