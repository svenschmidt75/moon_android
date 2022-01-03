package com.svenschmidt.kitana.di

import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import dagger.Component

@Component(modules = [DateTimeProviderModule::class])
interface ViewModelComponent {
    fun inject(dateTimeViewModel: DateTimeViewModel)
}