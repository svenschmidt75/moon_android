//! Calculate rise, set and transit times for the moon

use crate::date::date::Date;
use crate::date::jd::JD;
use crate::moon::position::{geocentric_latitude, geocentric_longitude};
use crate::refraction::refraction_for_true_altitude;
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;
use crate::{constants, coordinates, earth, ecliptic, moon};

pub(crate) enum OutputKind {
    Time(JD),
    NeverRises,
    NeverSets,
}

enum InputKind {
    Rise,
    Set,
    Transit,
}

/// Compute the time the moon rises
/// In:
/// date: Julian Day to compute the rise time for
/// timezone_offset: Observer's time zone offset
/// target_altitude: altitude of Moon above horizon, in degrees [-90, 90)
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
pub(crate) fn rise(
    jd: JD,
    timezone_offset: i8,
    target_altitude: Degrees,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
) -> OutputKind {
    calculate_rise_set_transit(
        InputKind::Rise,
        jd,
        timezone_offset,
        target_altitude,
        longitude_observer,
        latitude_observer,
    )
}

/// Compute the time the moon sets
/// In:
/// date: Julian Day to compute the rise time for
/// timezone_offset: Observer's time zone offset
/// target_altitude: altitude of Moon above horizon, in degrees [-90, 90)
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
pub(crate) fn set(
    jd: JD,
    timezone_offset: i8,
    target_altitude: Degrees,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
) -> OutputKind {
    calculate_rise_set_transit(
        InputKind::Set,
        jd,
        timezone_offset,
        target_altitude,
        longitude_observer,
        latitude_observer,
    )
}

/// Compute the time the moon transits (i.e. is in the meridian)
/// In:
/// date: Julian Day to compute the rise time for
/// timezone_offset: Observer's time zone offset
/// target_altitude: altitude of Moon above horizon, in degrees [-90, 90)
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
pub(crate) fn transit(
    jd: JD,
    timezone_offset: i8,
    target_altitude: Degrees,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
) -> OutputKind {
    calculate_rise_set_transit(
        InputKind::Transit,
        jd,
        timezone_offset,
        target_altitude,
        longitude_observer,
        latitude_observer,
    )
}

/// Compute the geocentric altitude of the Moon at rise/set.
/// It is defined to the that height at which the Moon's upper
/// limb touches the horizon.
/// In:
/// jd: Julian Day
/// altitude: Altitude at which to calculate the horizontal parallax effect for
/// (typically 0 deg)
/// longitude_observer: Observer's longitude, in degrees [-180, 180)
/// latitude_observer: Observer's latitude, in degrees [-90, 90)
/// pressure: Atmospheric pressure, in milibars. For atmospheric refraction effect
/// temperature: Ait temperature, in celsius. For atmospheric refraction effect
/// Out:
/// altitude, geocentric, at which the Moon's upper limb touches the observer's horizon,
/// in degrees [-90, 90). Typically, < 1 deg
pub(crate) fn target_altitude(
    jd: JD,
    altitude: Degrees,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    pressure: f64,
    temperature: f64,
) -> Degrees {
    // SS:Moon's horizontal parallax at 0 deg altitude (i.e. at the horizon)
    let parallax = moon::parallax::horizontal_parallax(jd, altitude);

    // SS: refraction effects
    let refraction = ArcSec::from(refraction_for_true_altitude(
        altitude,
        pressure,
        temperature,
    ));

    // SS: Moon's topocentric semidiameter
    let longitude = geocentric_longitude(jd);
    let latitude = geocentric_latitude(jd);
    let eps = ecliptic::true_obliquity(jd);
    let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);
    let theta0 = earth::apparent_siderial_time(jd);
    let theta = earth::local_siderial_time(theta0, longitude_observer);
    let hour_angle = (theta - ra).map_neg180_to_180();
    let semidiameter =
        moon::semidiameter::topocentric_semidiameter(jd, hour_angle, decl, latitude_observer, 0.0);

    let target_altitude_radians = Radians::from(parallax - refraction - semidiameter);
    Degrees::from(target_altitude_radians)
}

fn calculate_rise_set_transit(
    kind: InputKind,
    jd: JD,
    timezone_offset: i8,
    target_altitude: Degrees,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
) -> OutputKind {
    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();

    // SS: bound time based on observer's timezone offset
    let (jd_min, mut prev_jd, jd_max) = bound_julian_day(jd, timezone_offset);

    let sin_h0 = Radians::from(target_altitude).0.sin();

    // SS: if time change is less than a minute, we are done with iteration
    let delta_t_threshold = 1.0 / 60.0;

    let mut iter = 0;
    const MAX_ITER: u8 = 10;

    loop {
        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(prev_jd);
        let latitude = geocentric_latitude(prev_jd);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(prev_jd);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        let decl_radians = Radians::from(decl);
        let sin_decl = decl_radians.0.sin();
        let cos_decl = decl_radians.0.cos();
        let cos_hour_angle =
            (sin_h0 - sin_latitude_observer * sin_decl) / (cos_latitude_observer * cos_decl);

        let hour_angle;
        if cos_hour_angle < -1.0 {
            return OutputKind::NeverRises;
        } else if cos_hour_angle > 1.0 {
            return OutputKind::NeverSets;
        } else {
            hour_angle = Degrees::from(Radians::new(cos_hour_angle.acos()));
        }

        // SS: calculate the local hour angle for current time
        let theta0 = earth::apparent_siderial_time(prev_jd);
        let theta = earth::local_siderial_time(theta0, longitude_observer);
        let hour_angle2 = (theta - ra).map_neg180_to_180();

        let delta_hour_angle = match kind {
            InputKind::Rise => (hour_angle2 + hour_angle).map_neg180_to_180(),
            InputKind::Set => (hour_angle2 - hour_angle).map_neg180_to_180(),
            InputKind::Transit => hour_angle2,
        };

        // SS: convert degrees to solar time hours
        let delta_t = delta_hour_angle.to_hours() * constants::SIDERIAL_TO_SOLAR_TIME;

        // SS: correction step
        prev_jd.add_hours(-delta_t);

        if delta_t.abs() < delta_t_threshold || iter > MAX_ITER {
            break;
        }

        iter += 1;
    }

    // SS: check whether we have the correct day
    if prev_jd >= jd_min && prev_jd <= jd_max {
        OutputKind::Time(prev_jd)
    } else {
        match kind {
            InputKind::Rise => OutputKind::NeverRises,
            InputKind::Set => OutputKind::NeverSets,
            InputKind::Transit => {
                unreachable!()
            }
        }
    }
}

/// Calculate the min and max Julian Day the event has to be in
/// to be on the same day as the observer due to local time zone
/// offsets.
/// In:
/// jd: Julian Day to calculate the event for, in UTC
/// timezone_offset: Observer's time zone offset
fn bound_julian_day(jd: JD, timezone_offset: i8) -> (JD, JD, JD) {
    // SS: calculate midday UTC for the event
    let date = Date::from(jd);
    let midday = Date::new(date.year, date.month, date.day.trunc() + 0.5);
    let mut jd_midday = JD::from_date(midday);

    // SS: We return events (rise, set, transit) in UTC, so the caller has to convert
    // UTC to the local time by adding the timezone offset. Hence, here, we have to
    // subtract it to shift the 24 hour day window accordingly.
    jd_midday.add_hours(-timezone_offset as f64);

    // SS: calculate midnight in local time zone expressed in UTC
    let mut jd_min = jd_midday;
    jd_min.add_hours(-12.0);

    // SS: calculate midnight in local time zone expressed in UTC
    let mut jd_max = jd_midday;
    jd_max.add_hours(12.0);

    (jd_min, jd_midday, jd_max)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::date::date::Date;
    use crate::time;

    use super::*;

    #[test]
    fn timezone_offset_test_1() {
        // Arrange
        let date = Date::new(2022, 2, 21.0);
        let jd = JD::from_date(date);
        let timezone_offset = 1;

        // Act
        let (jd_min, jd_midday, jd_max) = bound_julian_day(jd, timezone_offset);

        // Assert
        let mut min = JD::from_date(date);
        min.add_hours(-timezone_offset as f64);
        assert_approx_eq!(min.jd, jd_min.jd, 0.001);

        let mut mid = min;
        mid.add_hours(12.0);
        assert_approx_eq!(mid.jd, jd_midday.jd, 0.001);

        let mut max = min;
        max.add_hours(24.0);
        assert_approx_eq!(max.jd, jd_max.jd, 0.001);
    }

    #[test]
    fn rise_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.5);
        let jd = JD::from_date(date);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        let target_altitude = target_altitude(
            jd,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // Act
        match rise(
            jd,
            0,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);
                println!(
                    "Date: {}/{}/{} {}:{}:{:.2}",
                    date.year,
                    date.month,
                    date.day.trunc() as u8,
                    h,
                    m,
                    s
                );

                // Assert
                let rise_date = Date::from_date_hms(2000, 3, 23, 21, 12, 13.0);
                let rise_date_jd = JD::from_date(rise_date);
                assert_approx_eq!(rise_date_jd.jd, jd.jd, 0.001)
            }

            OutputKind::NeverRises => {
                unreachable!()
            }

            OutputKind::NeverSets => {
                unreachable!()
            }
        }
    }

    #[test]
    fn rise_test_2() {
        // Arrange
        let date = Date::new(2000, 3, 25.5);
        let jd = JD::from_date(date);

        // SS: London, 0 deg, on Greenwich meridian
        let longitude_observer = Degrees::from_dms(0, 6, 3.2);
        let latitude_observer = Degrees::from_dms(51, 31, 54.8);

        let target_altitude = target_altitude(
            jd,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // Act
        if let OutputKind::NeverRises = rise(
            jd,
            0,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            // SS: The Moon does not rise in London on that day
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn rise_with_timezone_offset_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 25.0);
        let jd = JD::from_date(date);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        let target_altitude = target_altitude(
            jd,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // SS: Munich time is MEZ, i.e. +1 GMT
        let timezone_offset = 1;

        // Act
        match rise(
            jd,
            timezone_offset,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            OutputKind::Time(_) => {
                unreachable!()
            }

            OutputKind::NeverRises => {
                assert!(true);
            }

            OutputKind::NeverSets => {
                unreachable!()
            }
        }
    }

    #[test]
    fn rise_with_dynamical_time_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.5);
        let jd = JD::from_date(date);
        let tt = time::utc_2_tt(jd);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        let target_altitude = target_altitude(
            tt,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // Act
        match rise(
            tt,
            0,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);
                println!(
                    "Date: {}/{}/{} {}:{}:{:.2}",
                    date.year,
                    date.month,
                    date.day.trunc() as u8,
                    h,
                    m,
                    s
                );

                // Assert
                let rise_date = Date::from_date_hms(2000, 3, 23, 21, 12, 13.0);
                let rise_date_jd = JD::from_date(rise_date);
                assert_approx_eq!(rise_date_jd.jd, jd.jd, 0.001)
            }

            OutputKind::NeverRises => {
                unreachable!()
            }

            OutputKind::NeverSets => {
                unreachable!()
            }
        }
    }

    #[test]
    fn set_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.5);
        let jd = JD::from_date(date);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        let target_altitude = target_altitude(
            jd,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // Act
        match set(
            jd,
            0,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);
                println!(
                    "Date: {}/{}/{} {}:{}:{:.2}",
                    date.year,
                    date.month,
                    date.day.trunc() as u8,
                    h,
                    m,
                    s
                );

                // Assert
                let set_date = Date::from_date_hms(2000, 3, 23, 7, 1, 3.0);
                let set_date_jd = JD::from_date(set_date);
                assert_approx_eq!(set_date_jd.jd, jd.jd, 0.001)
            }

            OutputKind::NeverRises => {
                unreachable!()
            }

            OutputKind::NeverSets => {
                unreachable!()
            }
        }
    }

    #[test]
    fn set_test_2() {
        // Arrange
        let date = Date::new(2000, 4, 9.5);
        let jd = JD::from_date(date);

        // SS: London, 0 deg, on Greenwich meridian
        let longitude_observer = Degrees::from_dms(0, 6, 3.2);
        let latitude_observer = Degrees::from_dms(51, 31, 54.8);

        let target_altitude = target_altitude(
            jd,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // Act
        if let OutputKind::NeverSets = set(
            jd,
            0,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            // SS: The Moon does not rise in London on that day
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn transit_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.5);
        let jd = JD::from_date(date);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        let target_altitude = target_altitude(
            jd,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            1013.0,
            10.0,
        );

        // Act
        match transit(
            jd,
            0,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);
                println!(
                    "Date: {}/{}/{} {}:{}:{:.2}",
                    date.year,
                    date.month,
                    date.day.trunc() as u8,
                    h,
                    m,
                    s
                );

                // Assert
                let transit_date = Date::from_date_hms(2000, 3, 23, 1, 38, 1.0);
                let transit_date_jd = JD::from_date(transit_date);
                assert_approx_eq!(transit_date_jd.jd, jd.jd, 0.001)
            }

            OutputKind::NeverRises => {
                unreachable!()
            }

            OutputKind::NeverSets => {
                unreachable!()
            }
        }
    }
}
