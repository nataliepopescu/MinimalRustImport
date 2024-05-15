package com.example.minimalrustimport

class FooWrapper {
    val nativeFoo: Long;

    external fun newFoo(data1: Int, data2: Int): Long

    external fun getSum(nativePtr: Long): Int

    external fun getProd(nativePtr: Long): Int

    fun getSumWrapper(): Int {
        return getSum(nativeFoo)
    }

    fun getProdWrapper(): Int {
        return getProd(nativeFoo)
    }

    init {
        nativeFoo = newFoo(4, 4);
    }
}