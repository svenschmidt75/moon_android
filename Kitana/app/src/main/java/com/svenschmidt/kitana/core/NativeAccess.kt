package com.svenschmidt.kitana.core

import java.util.*

class NativeAccess {

    data class MoonData(
        var phaseAngle: Double = 0.0,
        var illuminatedFraction: Double = 0.0,
        var phaseDesc: String = "",
        var geocentricLongitude: Double = 0.0,
        var geocentricLatitude: Double = 0.0,
        var distanceFromEarth: Double = 0.0
    )

    companion object {
        // SS: format
        external fun rust_to_dms(degrees: Double, width: Byte): String

        // SS: Julian Day
        external fun rust_julian_day(year: Int, month: Int, day: Double): Double

        // SS: moon
        external fun rust_moon_data(jd: Double, moonData: MoonData)
    }
}