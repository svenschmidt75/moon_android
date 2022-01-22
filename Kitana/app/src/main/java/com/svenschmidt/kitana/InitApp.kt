package com.svenschmidt.kitana

import android.app.Application
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import com.svenschmidt.kitana.di.ViewModelComponent

class InitApp : Application() {
    private lateinit var appComponent: ViewModelComponent

    override fun onCreate() {
        super.onCreate()
        appComponent = DaggerViewModelComponent.builder().build()
    }

    fun appComp() = appComponent
}