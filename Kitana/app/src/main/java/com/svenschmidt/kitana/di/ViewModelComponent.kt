package com.svenschmidt.kitana.di

import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import com.svenschmidt.kitana.viewmodel.MoonActivityViewModel
import dagger.Component
import javax.inject.Singleton

@Singleton
@Component(modules = [DateTimeProviderModule::class])
interface ViewModelComponent {
    fun inject(viewModel: DateTimeViewModel)
    fun inject(viewModel: MoonActivityViewModel)
}