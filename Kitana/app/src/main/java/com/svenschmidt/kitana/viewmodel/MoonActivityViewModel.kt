package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.InitApp
import com.svenschmidt.kitana.core.DateTimeProvider
import com.svenschmidt.kitana.core.NativeAccess
import java.time.LocalDateTime
import java.time.ZoneId
import java.time.ZoneOffset
import java.time.ZonedDateTime
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
    val hourAngle = MutableLiveData<String>()
    val rightAscension = MutableLiveData<String>()
    val declination = MutableLiveData<String>()
    val altitude = MutableLiveData<String>()
    val azimuth = MutableLiveData<String>()
    val rises = MutableLiveData<String>()
    val transits = MutableLiveData<String>()
    val sets = MutableLiveData<String>()

    private fun Double.format(digits: Int) = "%.${digits}f".format(this)

    private fun Int.toHours(): Double = this / (60.0 * 60.0)

    init {
        // SS: inject Dagger dependencies
        (application.applicationContext as InitApp).vmComponent.inject(this)

        val subscriberToken = dateTimeProvider.subscribe { utcDateTime -> onUpdateDateTime(utcDateTime) }
        onUpdateDateTime(dateTimeProvider.getCurrentLocalDateTime())
    }

    private fun onUpdateDateTime(localDateTime: ZonedDateTime) {
        // SS: get UTC time
        val utcDateTime: LocalDateTime = localDateTime.toLocalDateTime().atZone(ZoneId.systemDefault()).withZoneSameInstant(ZoneOffset.UTC).toLocalDateTime()
        val (year, month, day) = DateTimeViewModel.fromLocalDateTime(utcDateTime)
        val julianDay = NativeAccess.rust_julian_day(year, month, day)

        // TODO: pass in correct lat, long, height -> location: Boulder, CO
        val timezoneOffset = localDateTime.offset.totalSeconds.toHours().toInt().toShort()
        val moonInputData = NativeAccess.MoonInputData(julianDay, timezoneOffset, 105.2151, 40.05972, 1624.0, 1013.0, 10.0)
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

        val hourAngleStr = NativeAccess.rust_to_hms(moonOutputData.hourAngle, 2)
        hourAngle.postValue(hourAngleStr)

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
