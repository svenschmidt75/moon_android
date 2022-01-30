package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.InitApp
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.core.NativeAccess
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
        // SS: inject Dagger dependencies
        (application.applicationContext as InitApp).vmComponent.inject(this)

        val subscriberToken = dateTimeProvider.subscribe { utcDateTime -> onUpdateDateTime(utcDateTime) }
        onUpdateDateTime(dateTimeProvider.getCurrentLocalDateTime())
    }

    private fun onUpdateDateTime(utcDateTime: LocalDateTime) {
        val (year, month, day) = DateTimeViewModel.fromLocalDateTime(utcDateTime)
        val julianDay = NativeAccess.rust_julian_day(year, month, day)

        // TODO: pass in correct lat, long, height
        val moonInputData = NativeAccess.MoonInputData(julianDay, 116.8625, 33.356111111111112, 1706.0, 1013.0, 10.0)
        val moonOutputData = NativeAccess.MoonOutputData()
        NativeAccess.rust_moon_data(moonInputData, moonOutputData);

        phaseAngle.postValue("${moonOutputData.phaseAngle.format(2)}°")
        fractionIlluminated.postValue("${(moonOutputData.illuminatedFraction * 100).format(2)}%")
        phaseName.postValue(moonOutputData.phaseDesc)

        val geocentricLongitudeDMS = NativeAccess.rust_to_dms(moonOutputData.geocentricLongitude, 2)
        geocentricLongitude.postValue(geocentricLongitudeDMS)

        val geocentricLatitudeDMS = NativeAccess.rust_to_dms(moonOutputData.geocentricLatitude, 2)
        geocentricLatitude.postValue(geocentricLatitudeDMS)

        distance.postValue("${moonOutputData.distanceFromEarth.format(0)}km")

        val raStr = NativeAccess.rust_to_hms(moonOutputData.rightAscension, 2)
        rightAscension.postValue(raStr)

        val declStr = NativeAccess.rust_to_dms(moonOutputData.declination, 2)
        declination.postValue(declStr)

        val azimuthStr = NativeAccess.rust_to_dms(moonOutputData.azimuth, 2)
        azimuth.postValue(azimuthStr)

        val altitudeStr = NativeAccess.rust_to_dms(moonOutputData.altitude, 2)
        altitude.postValue(altitudeStr)

        rises.postValue("Waning Crescent")
        transits.postValue("175.365")
        sets.postValue("75.365")
    }


}