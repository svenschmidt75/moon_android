package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.InitApp
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.core.NativeAccess
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
//        DaggerViewModelComponent.builder().build().inject(this)

        (application.applicationContext as InitApp).appComp.inject(this)

        // SS: initialize UI with current date/time
        val dateTime = dateTimeProvider.getCurrentLocalDateTime()
        updateDateTime(dateTime)

        // SS TODO: when activity stops or is suspended, ...
        val subscriberToken = dateTimeProvider.subscribe {  now -> updateDateTime(now as LocalDateTime) }
    }

    companion object {

        data class FromLocalDateTimeResult(val year: Int, val month: Int, val day: Double)

        fun fromLocalDateTime(dateTime: LocalDateTime): FromLocalDateTimeResult {
            var day = dateTime.dayOfMonth.toDouble()

            val dayFraction =
                (dateTime.hour + dateTime.minute / 60.0 + dateTime.second / (60.0 * 60.0)) / 24.0
            day += dayFraction

            return FromLocalDateTimeResult(dateTime.year, dateTime.monthValue, day)
        }

    }

    fun onUpdateDateTime() {
        if (updateDateTime.value!!) {
            dateTimeProvider.start()
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
                .toLocalDateTime()
        formatted = utcDateTime.format(
            DateTimeFormatter.ofPattern(
                "yyyy-MMM-dd HH:mm:ss",
                Locale.getDefault()
            )
        )
        utcTime.postValue(formatted)

        // SS: set Julian day
        val (year, month, day) = fromLocalDateTime(utcDateTime)
        val julianDay = NativeAccess.rust_julian_day(year, month, day)
        this.julianDay.postValue(julianDay.toString())
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