package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import java.time.LocalDateTime
import java.time.ZoneId
import java.time.ZoneOffset
import java.time.format.DateTimeFormatter
import java.util.*
import javax.inject.Inject

class DateTimeViewModel(application: Application) : AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

    val updateDateTime = MutableLiveData<Boolean>()
    val localTime = MutableLiveData<String>()
    val utcTime = MutableLiveData<String>()
    val julianDay = MutableLiveData<String>()

    init {
        DaggerViewModelComponent.builder().build().inject(this)

        // SS: initialize UI with current date/time
        val dateTime = dateTimeProvider.getCurrentLocalDateTime()
        updateDateTime(dateTime)
    }

    fun onUpdateDateTime() {
        if (updateDateTime.value!!) {
            dateTimeProvider.start { _, now ->
                updateDateTime(now as LocalDateTime)
            }
        } else {
            dateTimeProvider.stop()
        }
    }

    private fun updateDateTime(dateTime: LocalDateTime) {
        // SS: get local time
        var formatted = dateTime.format(
            DateTimeFormatter.ofPattern(
                "yyyy-MMM-dd HH:mm:ss",
                Locale.getDefault()
            )
        )
        localTime.postValue(formatted)

        // SS: get UTC time
        val utcDateTime =
            dateTime.atZone(ZoneId.systemDefault()).withZoneSameInstant(ZoneOffset.UTC)
                .toLocalDateTime();
        formatted = utcDateTime.format(
            DateTimeFormatter.ofPattern(
                "yyyy-MMM-dd HH:mm:ss",
                Locale.getDefault()
            )
        )
        utcTime.postValue(formatted)
    }

    fun setDate(year: Int, month: Int, dayOfMonth: Int) {
        val dateTime = dateTimeProvider.getCurrentLocalDateTime()
        val updatedDateTime = LocalDateTime.of(
            year,
            month + 1,
            dayOfMonth,
            dateTime.hour,
            dateTime.minute,
            dateTime.second
        )
        dateTimeProvider.setCurrentLocalDateTime(updatedDateTime)

        // SS: update UI
        updateDateTime(updatedDateTime)
    }

    fun setTime(hourOfDay: Int, minute: Int) {
        val dateTime = dateTimeProvider.getCurrentLocalDateTime()
        val updatedDateTime = LocalDateTime.of(
            dateTime.year,
            dateTime.month,
            dateTime.dayOfMonth,
            hourOfDay,
            minute,
            dateTime.second
        )
        dateTimeProvider.setCurrentLocalDateTime(updatedDateTime)

        // SS: update UI
        updateDateTime(updatedDateTime)
    }

    fun setCurrentDateTime() {
        val dateTime = dateTimeProvider.getSystemLocalDateTime()
        dateTimeProvider.setCurrentLocalDateTime(dateTime)

        // SS: update UI
        updateDateTime(dateTime)
    }

}