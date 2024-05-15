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

        val fooWrapper = FooWrapper()
        binding.fooPtr.text = java.lang.Long.toHexString(fooWrapper.nativeFoo)

        // The following three methods all get the same result in slightly different ways
        binding.fooSumMain.text = getSum(fooWrapper.nativeFoo).toString()
        binding.fooSumWrapperFromMain.text = fooWrapper.getSum(fooWrapper.nativeFoo).toString()
        binding.fooSumWrapper.text = fooWrapper.getSumWrapper().toString()

        binding.fooProd.text = fooWrapper.getProdWrapper().toString()
    }

    external fun hello(): String?

    external fun add(left: Int, right: Int): Int

    external fun getSum(nativePtr: Long): Int

    companion object {
        init {
            System.loadLibrary("simple")
        }
    }
}
