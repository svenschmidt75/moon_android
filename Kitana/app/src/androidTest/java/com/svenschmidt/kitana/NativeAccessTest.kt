package com.svenschmidt.kitana

import androidx.test.ext.junit.runners.AndroidJUnit4
import com.svenschmidt.kitana.core.NativeAccess
import com.svenschmidt.kitana.viewmodel.DateTimeViewModel
import org.hamcrest.MatcherAssert
import org.hamcrest.number.IsCloseTo.closeTo
import org.junit.Assert
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import java.time.LocalDateTime

@RunWith(AndroidJUnit4::class)
class NativeAccessTest {
    @Before
    fun setUp() {
        // SS: load native libraries
        System.loadLibrary("moon_android")
    }

    @Test
    fun rust_julian_day_test() {
        // Arrange
        val (year, month, day) =  DateTimeViewModel.fromLocalDateTime(LocalDateTime.of(2022, 1, 16, 14, 26, 18))

        // Act
        val jd = NativeAccess.rust_julian_day(year, month, day)

        // Assert
        MatcherAssert.assertThat(jd, closeTo(2_459_596.101598, 0.000_001))
    }

    @Test
    fun rust_to_dms() {
        // Arrange
        val degrees = 13.769657226951539

        // Act
        val dms_str = NativeAccess.rust_to_dms(degrees, 3);

        // Assert
        Assert.assertEquals(dms_str, "13° 46' 10.766\"")
    }

    @Test
    fun rust_moon_data_test() {
        // Arrange

        // SS: Jan. 16th 2022, 2:26:18pm UT
        val jd = 2_459_596.101598

        // Act
        val moonData = NativeAccess.MoonData(1.0)
        NativeAccess.rust_moon_data(jd, moonData);

        // Assert
        MatcherAssert.assertThat(moonData.phaseAngle, closeTo(164.6175559076692, 0.000_001))
        MatcherAssert.assertThat(moonData.illuminatedFraction, closeTo(0.981, 0.001))
        Assert.assertEquals(moonData.phaseDesc, "Full Moon")
    }


}