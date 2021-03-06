package com.svenschmidt.kitana.core

class NativeAccess {

    data class DateTime(
        var isValid: Boolean = false,
        var year: Short = 0,
        var month: Short = 0,
        var day: Short = 0,
        var hours: Short = 0,
        var minutes: Short = 0,
        var seconds: Double = 0.0,
    )

    data class MoonOutputData(
        var phaseAngle: Double = 0.0,
        var phaseAge: Double = 0.0,
        var illuminatedFraction: Double = 0.0,
        var phaseDesc: String = "",
        var geocentricLongitude: Double = 0.0,
        var geocentricLatitude: Double = 0.0,
        var distanceFromEarth: Double = 0.0,
        var hourAngle: Double = 0.0,
        var rightAscension: Double = 0.0,
        var declination: Double = 0.0,
        var azimuth: Double = 0.0,
        var altitude: Double = 0.0,
        var riseTime: DateTime = DateTime(),
        var setTime: DateTime = DateTime(),
        var transitTime: DateTime = DateTime(),
    )

    data class MoonInputData(
        var jd: Double,
        var timezoneOffset: Short,
        var longitudeObserver: Double,
        var latitudeObserver: Double,
        var heightAboveSeaObserver: Double,
        var pressure: Double,
        var temperature: Double,
    )

    companion object {
        // SS: format
        external fun rust_to_dms(degrees: Double, width: Byte): String
        external fun rust_to_hms(degrees: Double, width: Byte): String

        // SS: Julian Day
        external fun rust_julian_day(year: Int, month: Int, day: Double): Double

        // SS: Siderial Time
        external fun rust_local_siderial_time(jd: Double, longitudeObserver: Double): Double

        // SS: moon
        external fun rust_moon_data(moonInputData: MoonInputData, moonOutputData: MoonOutputData)
    }
}