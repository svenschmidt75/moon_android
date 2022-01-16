package com.svenschmidt.kitana.di

import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import com.svenschmidt.kitana.viewmodel.MoonActivityViewModel
import dagger.Component

@Component(modules = [DateTimeProviderModule::class])
interface ViewModelComponent {
    fun inject(viewModel: DateTimeViewModel)
    fun inject(viewModel: MoonActivityViewModel)
}