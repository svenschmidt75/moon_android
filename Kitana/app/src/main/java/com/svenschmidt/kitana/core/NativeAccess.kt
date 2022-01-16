package com.svenschmidt.kitana.core

class NativeAccess {

    data class MoonData(
        var phaseAngle: Double
    )

    companion object {
        // SS: Julian Day
        external fun rust_julian_day(year: Int, month: Int, day: Double): Double

        // SS: moon
        external fun rust_moon_data(jd: Double, moonData: MoonData)
    }
}