package com.svenschmidt.kitana.core

class NativeAccess {

    data class MoonOutputData(
        var phaseAngle: Double = 0.0,
        var illuminatedFraction: Double = 0.0,
        var phaseDesc: String = "",
        var geocentricLongitude: Double = 0.0,
        var geocentricLatitude: Double = 0.0,
        var distanceFromEarth: Double = 0.0,
        var rightAscension: Double = 0.0,
        var declination: Double = 0.0,
        var azimuth: Double = 0.0,
        var altitude: Double = 0.0
    )

    data class MoonInputData(
        var jd: Double,
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