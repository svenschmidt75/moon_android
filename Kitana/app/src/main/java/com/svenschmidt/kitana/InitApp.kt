package com.svenschmidt.kitana

import android.app.Application
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import com.svenschmidt.kitana.di.ViewModelComponent

open class InitApp : Application() {

    val appComp: ViewModelComponent by lazy {
        initializeComponent()
    }

    open fun initializeComponent(): ViewModelComponent {
        // Creates an instance of AppComponent using its Factory constructor
        // We pass the applicationContext that will be used as Context in the graph
        return DaggerViewModelComponent.factory().create(applicationContext)
    }

}