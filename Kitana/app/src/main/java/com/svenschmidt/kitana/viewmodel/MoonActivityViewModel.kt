package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.MutableLiveData
import com.svenschmidt.kitana.core.DateTimeProvider
import javax.inject.Inject

class MoonActivityViewModel(application: Application) : AndroidViewModel(application) {

    @Inject
    lateinit var dateTimeProvider: DateTimeProvider

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


    init {
        fractionIlluminated.postValue("56.4%")
        phaseName.postValue("Waning Crescent")
        geocentricLongitude.postValue("175.365")
        geocentricLatitude.postValue("75.365")
        distance.postValue("56.4%")
        rightAscension.postValue("Waning Crescent")
        declination.postValue("175.365")
        altitude.postValue("75.365")
        azimuth.postValue("56.4%")
        rises.postValue("Waning Crescent")
        transits.postValue("175.365")
        sets.postValue("75.365")
    }


}