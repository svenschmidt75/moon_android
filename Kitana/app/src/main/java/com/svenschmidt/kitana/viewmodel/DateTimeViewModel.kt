package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import java.text.SimpleDateFormat
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.util.*
import javax.inject.Inject

class DateTimeViewModel(application: Application): AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

    val updateDateTime = MutableLiveData<Boolean>()
    val localTime = MutableLiveData<String>()
    val utcTime = MutableLiveData<String>()

    init {
        DaggerViewModelComponent.builder().build().inject(this)
    }

    var selectDateTimeEnabled: Boolean = true

    fun onUpdateDateTime() {
        if (updateDateTime.value!!) {
            dateTimeProvider.start { _, now ->
                val nowUTC = now as Long

                // SS: get local time
                val formatter = SimpleDateFormat("yyyy-MMM-dd HH:mm:ss", Locale.getDefault())
                var formatted = formatter.format(Date(nowUTC))
                localTime.postValue(formatted)

                // SS: get UTC time
                formatter.timeZone = TimeZone.getTimeZone("UTC")
                formatted = formatter.format(Date(nowUTC))
                utcTime.postValue(formatted)
            }
        }
        else {
            dateTimeProvider.stop()
        }

    }


}