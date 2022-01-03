package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.Observer
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import java.text.SimpleDateFormat
import java.util.*
import javax.inject.Inject

class DateTimeViewModel(application: Application): AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

    val useCurrentTime = MutableLiveData<Boolean>()
    val utc = MutableLiveData<String>()

    init {
        DaggerViewModelComponent.builder().build().inject(this)
    }

    fun onUseCurrentTime() {
        dateTimeProvider.start(java.util.Observer { _, date ->
            utc.postValue(date as String)
        })
    }


}