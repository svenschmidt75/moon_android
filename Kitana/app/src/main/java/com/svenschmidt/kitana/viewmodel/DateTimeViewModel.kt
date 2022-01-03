package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import javax.inject.Inject

class DateTimeViewModel(application: Application): AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

    init {
        DaggerViewModelComponent.builder().build().inject(this)
    }
}