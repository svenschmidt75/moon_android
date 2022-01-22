package com.svenschmidt.kitana.di

import android.content.Context
import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import com.svenschmidt.kitana.viewmodel.MoonActivityViewModel
import dagger.BindsInstance
import dagger.Component
import javax.inject.Singleton

@Singleton
@Component(modules = [DateTimeProviderModule::class])
interface ViewModelComponent {

    // Factory to create instances of the ViewModelComponent
    @Component.Factory
    interface Factory {
        // With @BindsInstance, the Context passed in will be available in the graph
        fun create(@BindsInstance context: Context): ViewModelComponent
    }

    fun inject(viewModel: DateTimeViewModel)
    fun inject(viewModel: MoonActivityViewModel)
}