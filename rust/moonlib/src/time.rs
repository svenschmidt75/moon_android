//! Time-related function.
//!
use crate::date::jd::JD;
use crate::{constants, util};
use tabular::time::delta_t_data::{DeltaTValue, DELTA_T_DATA};
use tabular::time::leap_second_data::{LeapSecondCoefficient, LEAP_SECOND_DATA};

/// Calculate the amount of leap seconds for the date passed in.
/// This is to calculate TAI from UTC, i.e. TAI - UTC = cumulative_leap_seconds(UTC)
/// In: Julian Day, in UTC
/// Out: cumulative leap seconds for input date
pub fn cumulative_leap_seconds(jd: JD) -> f64 {
    let mut cumulative_leap_secs = 0.0;

    let mut idx = LEAP_SECOND_DATA.len();

    if jd.jd >= LEAP_SECOND_DATA[0].jd {
        if jd.jd < LEAP_SECOND_DATA[idx - 1].jd {
            let to_find = LeapSecondCoefficient {
                jd: jd.jd,
                leap_seconds: 0.0,
                base_mjd: 0.0,
                coefficient: 0.0,
            };
            idx = util::binary_search::upper_bound(&LEAP_SECOND_DATA, &to_find);
        }

        let leap_item = &LEAP_SECOND_DATA[idx - 1];
        cumulative_leap_secs = leap_item.leap_seconds
            + (jd.to_mjd() - JD::new(leap_item.base_mjd)).jd * leap_item.coefficient;
    }

    cumulative_leap_secs
}

/// Calculate the correction delta_t between UT1 and TT, i.e.
/// TT - UT1 = delta_t
/// In: Julian Day in UTC
/// Out: delta_t, in seconds
fn delta_t(jd: JD) -> f64 {
    let delta_t;

    if jd.jd >= DELTA_T_DATA[0].jd && jd.jd < DELTA_T_DATA[DELTA_T_DATA.len() - 1].jd {
        // SS: calculate delta_t by using tabular data from
        // https://cddis.nasa.gov/archive/products/iers/historic_deltat.data
        // and
        // https://cddis.nasa.gov/archive/products/iers/finals2000A.all

        let to_find = DeltaTValue {
            jd: jd.jd,
            delta_t: 0.0,
        };
        let idx = util::binary_search::upper_bound(&DELTA_T_DATA, &to_find);

        let prev = &DELTA_T_DATA[idx - 1];
        let curr = &DELTA_T_DATA[idx];

        delta_t =
            (jd.jd - prev.jd) / (curr.jd - prev.jd) * (curr.delta_t - prev.delta_t) + prev.delta_t;
    } else {
        // SS: Julian Day outside of tabular data range, calculate delta_t based on
        // polynomial expressions from Espenak & Meeus 2006.
        // References: http://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html and
        // http://www.staff.science.uu.nl/~gent0113/deltat/deltat_old.htm,
        // see Espenak & Meeus 2006 section at the bottom
        let date = jd.to_calendar_date();
        let y = date.fractional_year().trunc() as i16;

        if y < -500 {
            let u = (y as f64 - 1820.0) / 100.0;
            let u2 = u * u;
            delta_t = -20.0 + (32.0 * u2);
        } else if y < 500 {
            let u = y as f64 / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            let u6 = u5 * u;
            delta_t = 10583.6
                + (-1014.41 * u)
                + (33.78311 * u2)
                + (-5.952053 * u3)
                + (-0.1798452 * u4)
                + (0.022174192 * u5)
                + (0.0090316521 * u6);
        } else if y < 1600 {
            let u = (y as f64 - 1000.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            let u6 = u5 * u;
            delta_t = 1574.2
                + (-556.01 * u)
                + (71.23472 * u2)
                + (0.319781 * u3)
                + (-0.8503463 * u4)
                + (-0.005050998 * u5)
                + (0.0083572073 * u6);
        } else if y < 1700 {
            let u = (y as f64 - 1600.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta_t = 120.0 + (-98.08 * u) + (-153.2 * u2) + (u3 / 0.007129);
        } else if y < 1800 {
            let u = (y as f64 - 1700.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            delta_t = 8.83 + (16.03 * u) + (-59.285 * u2) + (133.36 * u3) + (-u4 / 0.01174);
        } else if y < 1860 {
            let u = (y as f64 - 1800.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            let u6 = u5 * u;
            let u7 = u6 * u;
            delta_t = 13.72
                + (-33.2447 * u)
                + (68.612 * u2)
                + (4111.6 * u3)
                + (-37436.0 * u4)
                + (121272.0 * u5)
                + (-169900.0 * u6)
                + (87500.0 * u7);
        } else if y < 1900 {
            let u = (y as f64 - 1860.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            delta_t = 7.62
                + (57.37 * u)
                + (-2517.54 * u2)
                + (16806.68 * u3)
                + (-44736.24 * u4)
                + (u5 / 0.0000233174);
        } else if y < 1920 {
            let u = (y as f64 - 1900.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            delta_t = -2.79 + (149.4119 * u) + (-598.939 * u2) + (6196.6 * u3) + (-19700.0 * u4);
        } else if y < 1941 {
            let u = (y as f64 - 1920.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta_t = 21.20 + (84.493 * u) + (-761.00 * u2) + (2093.6 * u3);
        } else if y < 1961 {
            let u = (y as f64 - 1950.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta_t = 29.07 + (40.7 * u) + (-u2 / 0.0233) + (u3 / 0.002547);
        } else if y < 1986 {
            let u = (y as f64 - 1975.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta_t = 45.45 + 106.7 * u - u2 / 0.026 - u3 / 0.000718;
        } else if y < 2005 {
            let u = (y as f64 - 2000.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            delta_t = 63.86
                + (33.45 * u)
                + (-603.74 * u2)
                + (1727.5 * u3)
                + (65181.4 * u4)
                + (237359.9 * u5);
        } else if y < 2050 {
            let u = (y as f64 - 2000.0) / 100.0;
            let u2 = u * u;
            delta_t = 62.92 + (32.217 * u) + (55.89 * u2);
        } else if y < 2150 {
            let u = (y as f64 - 1820.0) / 100.0;
            let u2 = u * u;
            delta_t = -205.72 + (56.28 * u) + (32.0 * u2);
        } else {
            let u = (y as f64 - 1820.0) / 100.0;
            let u2 = u * u;
            delta_t = -20.0 + (32.0 * u2);
        }
    }

    delta_t
}

/// Convert UTC to TT
/// In: Julian Day, in UTC
/// Out: TT, in seconds
fn utc_2_tt(jd: JD) -> JD {
    // SS: If the date falls outside the range we have leap second data for, we
    // interpret the input date in UT1 rather than UTC. Same as PJ Naughter
    if jd.jd < LEAP_SECOND_DATA[0].jd || jd.jd > LEAP_SECOND_DATA.last().unwrap().jd {
        ut1_to_tt(jd)
    } else {
        let delta_t = delta_t(jd);
        let cumulative_leap_seconds = cumulative_leap_seconds(jd);

        // SS: calculate UT1 from UTC
        let ut1 =
            jd.jd - (-delta_t - cumulative_leap_seconds - 32.184) / constants::SEC_PER_DAY as f64;

        // SS: calculate TT from UT1
        let jd_in_tt = ut1 + delta_t / constants::SEC_PER_DAY as f64;
        JD::new(jd_in_tt)
    }
}

/// Convert UT1 to T(erestial) T(ime)
/// In: Julian Day, in UT1
/// Out: TT, in seconds
fn ut1_to_tt(jd: JD) -> JD {
    let delta_t = delta_t(jd);

    // SS: Julian Day is in units of days, so convert
    // delta_t from seconds to days
    let delta_t_in_days = delta_t / constants::SEC_PER_DAY as f64;

    JD::new(jd.jd + delta_t_in_days)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::date::Date;
    use crate::date::jd::JD;
    use crate::earth::{
        apparent_siderial_time, hour_angle, local_siderial_time, mean_siderial_time,
    };
    use crate::util::degrees::Degrees;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn utc_to_tt_test() {
        // Arrange
        let jd_in_utc = [
            JD::new(2457754.5),
            JD::new(2459610.080526),
            JD::from_date(Date::from_date_hms(2003, 8, 28, 3, 17, 0.0)),
            JD::from_date(Date::from_date_hms(1947, 3, 15, 7, 17, 56.0)),
        ];
        let tt_expected = [
            2457754.502388,
            2459610.0813279948,
            2452879.639042,
            2432259.804442991,
        ];

        for i in 0..jd_in_utc.len() {
            let jd = jd_in_utc[i];
            let dt = tt_expected[i];

            // Act
            let tt = utc_2_tt(jd);

            // Assert
            assert_approx_eq!(dt, tt.jd, 0.000_001);
        }
    }

    #[test]
    fn delta_t_test() {
        // Arrange
        let jd_in_utc = [
            JD::new(2457754.5),
            JD::new(2459610.080526),
            JD::from_date(Date::from_date_hms(2003, 8, 28, 3, 17, 0.0)),
            JD::from_date(Date::from_date_hms(1947, 3, 15, 7, 17, 56.0)),
        ];
        let delta_t_expected = [68.5927179, 69.2917789, 64.533476, 27.874433];

        for i in 0..jd_in_utc.len() {
            let jd = jd_in_utc[i];
            let dt = delta_t_expected[i];

            // Act
            let delta_t = delta_t(jd);

            // Assert
            assert_approx_eq!(dt, delta_t, 0.000_001);
        }
    }

    #[test]
    fn cumulative_leap_seconds_test1() {
        // Arrange
        let jd = JD::from_date(Date::from_date_hms(2003, 8, 28, 3, 17, 0.0));

        // Act
        let leap_seconds = cumulative_leap_seconds(jd);

        // Assert
        assert_approx_eq!(32.0, leap_seconds, 0.1)
    }

    #[test]
    fn cumulative_leap_seconds_test2() {
        // Arrange
        let jd = JD::new(2_457_754.5);

        // Act
        let leap_seconds = cumulative_leap_seconds(jd);

        // Assert
        assert_approx_eq!(37.0, leap_seconds, 0.1)
    }

    #[test]
    fn hour_angle_test() {
        // Meeus, page 95, example 13.b

        // Arrange
        let siderial_time_apparent_greenwich = Degrees::from_hms(8, 34, 56.853);
        let longitude_observer = Degrees::from_hms(5, 8, 15.7);
        let right_ascension_apparent = Degrees::from_hms(23, 9, 16.641);

        // Act
        let siderial_time_local =
            local_siderial_time(siderial_time_apparent_greenwich, longitude_observer);
        let hour_angle = hour_angle(siderial_time_local, right_ascension_apparent);

        // Assert
        assert_approx_eq!(64.352133, hour_angle.0, 0.00001)
    }

    #[test]
    fn local_siderial_time_test_1() {
        // Arrange

        // SS: Jan 29th, 2022, 2:32:20pm UTC
        let jd = JD::new(2_459_609.105793);

        let longitude_observer = Degrees::from_dms(105, 12, 53.8);

        let mean_siderial_time = mean_siderial_time(jd);

        // Act
        let theta0 = local_siderial_time(mean_siderial_time, longitude_observer);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 16);
        assert_eq!(m, 6);
        assert_approx_eq!(46.9, s, 0.1)
    }

    #[test]
    fn mean_siderial_time_test_1() {
        // Arrange

        // SS: Jan 16th, 2022, 2:26:18pm UTC
        let jd = JD::new(2_459_596.101598);

        // Act
        let theta0 = mean_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 22);
        assert_eq!(m, 10);
        assert_approx_eq!(19.92073, s, 0.00001)
    }

    #[test]
    fn mean_siderial_time_test_2() {
        // Meeus, example 12.b, page 89

        // Arrange

        // SS: Apr. 10th 1987, 19h:21m:00s UT
        let jd = JD::new(2_446_896.30625);

        // Act
        let theta0 = mean_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_approx_eq!(128.7378734, theta0.0, 0.00001);

        assert_eq!(h, 8);
        assert_eq!(m, 34);
        assert_approx_eq!(57.0896, s, 0.0001)
    }

    #[test]
    fn apparent_siderial_time_test_1() {
        // Arrange

        // SS: Jan 16th, 2022, 2:26:18pm UTC
        let jd = JD::new(2_459_596.101598);

        // Act
        let theta0 = apparent_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 22);
        assert_eq!(m, 10);
        assert_approx_eq!(19.10356, s, 0.00001)
    }

    #[test]
    fn apparent_siderial_time_test_2() {
        // Meeus, example 12.a, page 88

        // Arrange

        // SS: Apr. 10th 1987, 0 UT
        let jd = JD::new(2_446_895.5);

        // Act
        let theta0 = apparent_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 13);
        assert_eq!(m, 10);
        assert_approx_eq!(46.1351, s, 0.000_1)
    }
}
