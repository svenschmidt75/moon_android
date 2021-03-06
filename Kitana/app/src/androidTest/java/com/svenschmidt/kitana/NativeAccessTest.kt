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
        System.loadLibrary("moonlib")
    }

    @Test
    fun rust_julian_day_test() {
        // Arrange
        val (year, month, day) = DateTimeViewModel.fromLocalDateTime(
            LocalDateTime.of(
                2022,
                1,
                16,
                14,
                26,
                18
            )
        )

        // Act
        val jd = NativeAccess.rust_julian_day(year, month, day)

        // Assert
        MatcherAssert.assertThat(jd, closeTo(2_459_596.101598, 0.000_001))
    }

    @Test
    fun rust_local_siderial_time_test() {
        // Arrange
        val (year, month, day) = DateTimeViewModel.fromLocalDateTime(
            LocalDateTime.of(
                2022,
                1,
                16,
                14,
                26,
                18
            )
        )
        val jd = NativeAccess.rust_julian_day(year, month, day)

        // SS: Mount Palomar longitude
        val longitudeObserver = -116.8649959122331;

        // Act
        val siderialTime = NativeAccess.rust_local_siderial_time(jd, longitudeObserver)

        // Assert
        MatcherAssert.assertThat(siderialTime, closeTo(89.44431320364197, 0.000_001))
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
    fun rust_to_hms() {
        // Arrange
        val degrees = 241.6958092513155

        // Act
        val hms_str = NativeAccess.rust_to_hms(degrees, 3);

        // Assert
        Assert.assertEquals(hms_str, "16h 6m 46.994s")
    }

    @Test
    fun rust_moon_data_test() {
        // Arrange

        // SS: Jan. 16th 2022, 2:26:18pm UT
        val jd = 2_459_596.101598

        // SS: Mount Palomar longitude
        val longitudeObserver = 116.8625;

        // SS: Mount Palomar latitude
        val latitudeObserver = 33.356111111111112;

        // SS: Mount Palomar height above sea level
        val heightAboveSea = 1706.0;

        // SS: local timezone offset from GMT
        val timezone_offset: Short = -7;

        // Act
        val moonInputData =
            NativeAccess.MoonInputData(jd, timezone_offset, longitudeObserver, latitudeObserver, heightAboveSea, 1013.0, 10.0)
        val moonOutputData = NativeAccess.MoonOutputData()
        NativeAccess.rust_moon_data(moonInputData, moonOutputData);

        // Assert
        MatcherAssert.assertThat(moonOutputData.phaseAngle, closeTo(164.6175559076692, 0.000_001))
        MatcherAssert.assertThat(moonOutputData.illuminatedFraction, closeTo(0.981, 0.001))
        Assert.assertEquals(moonOutputData.phaseDesc, "Full Moon")
        MatcherAssert.assertThat(
            moonOutputData.distanceFromEarth,
            closeTo(403836.9196467576, 0.001)
        )
        MatcherAssert.assertThat(moonOutputData.hourAngle, closeTo(114.1696980754786, 0.001))
        MatcherAssert.assertThat(
            moonOutputData.geocentricLatitude,
            closeTo(3.3226976900929386, 0.001)
        )
        MatcherAssert.assertThat(
            moonOutputData.geocentricLongitude,
            closeTo(101.04539708087002, 0.001)
        )
        MatcherAssert.assertThat(moonOutputData.rightAscension, closeTo(101.54739997724147, 0.001))
        MatcherAssert.assertThat(moonOutputData.declination, closeTo(25.711018001886877, 0.001))
        MatcherAssert.assertThat(moonOutputData.azimuth, closeTo(124.51199928817729, 0.001))
        MatcherAssert.assertThat(moonOutputData.altitude, closeTo(-3.2443418232251013, 0.001))
    }


}