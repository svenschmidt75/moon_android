package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.InitApp
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.core.NativeAccess
import com.svenschmidt.kitana.di.DaggerViewModelComponent
import java.time.LocalDateTime
import javax.inject.Inject

class MoonActivityViewModel(application: Application) : AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

    val phaseAngle = MutableLiveData<String>()
    val fractionIlluminated = MutableLiveData<String>()
    val phaseName = MutableLiveData<String>()
    val geocentricLongitude = MutableLiveData<String>()
    val geocentricLatitude = MutableLiveData<String>()
    val distance = MutableLiveData<String>()
    val rightAscension = MutableLiveData<String>()
    val declination = MutableLiveData<String>()
    val altitude = MutableLiveData<String>()
    val azimuth = MutableLiveData<String>()
    val rises = MutableLiveData<String>()
    val transits = MutableLiveData<String>()
    val sets = MutableLiveData<String>()

    fun Double.format(digits: Int) = "%.${digits}f".format(this)

    init {
        //DaggerViewModelComponent.builder().build().inject(this)

        (application.applicationContext as InitApp).appComp.inject(this)

        val subscriberToken = dateTimeProvider.subscribe { utcDateTime -> onUpdateDateTime(utcDateTime) }
        onUpdateDateTime(dateTimeProvider.getCurrentLocalDateTime())
    }

    private fun onUpdateDateTime(utcDateTime: LocalDateTime) {
        val (year, month, day) = DateTimeViewModel.fromLocalDateTime(utcDateTime)
        val julianDay = NativeAccess.rust_julian_day(year, month, day)

        val moonData = NativeAccess.MoonData()
        NativeAccess.rust_moon_data(julianDay, moonData);

        phaseAngle.postValue("${moonData.phaseAngle.format(2)}°")
        fractionIlluminated.postValue("${(moonData.illuminatedFraction * 100).format(2)}%")
        phaseName.postValue(moonData.phaseDesc)

        val geocentricLongitudeDMS = NativeAccess.rust_to_dms(moonData.geocentricLongitude, 2)
        geocentricLongitude.postValue(geocentricLongitudeDMS)

        val geocentricLatitudeDMS = NativeAccess.rust_to_dms(moonData.geocentricLatitude, 2)
        geocentricLatitude.postValue(geocentricLatitudeDMS)

        distance.postValue("${moonData.distanceFromEarth.format(0)}km")

        rightAscension.postValue("Waning Crescent")
        declination.postValue("175.365")
        altitude.postValue("75.365")
        azimuth.postValue("56.4%")
        rises.postValue("Waning Crescent")
        transits.postValue("175.365")
        sets.postValue("75.365")
    }


}