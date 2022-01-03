package com.svenschmidt.kitana.di

import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.core.DateTimeProviderImpl
import dagger.Module
import dagger.Provides

@Module
class DateTimeProviderModule {
    @Provides
    fun providesDateTime(): DateTimeProvider = DateTimeProviderImpl()
}