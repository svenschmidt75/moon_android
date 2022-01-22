package com.svenschmidt.kitana.di

import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.core.DateTimeProviderImpl
import dagger.Module
import dagger.Provides
import javax.inject.Singleton

@Module
class DateTimeProviderModule {
    @Provides
    @Singleton
    fun providesDateTime(): DateTimeProvider = DateTimeProviderImpl()
}