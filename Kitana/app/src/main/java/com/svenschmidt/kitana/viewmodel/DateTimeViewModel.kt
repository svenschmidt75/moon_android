package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.Observer
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import javax.inject.Inject

class DateTimeViewModel(application: Application): AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

    val useCurrentTime = MutableLiveData<Boolean>()

    init {
        DaggerViewModelComponent.builder().build().inject(this)
    }



}