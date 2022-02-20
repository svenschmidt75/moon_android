//! Calculate rise, set and transit times for the moon

use crate::{constants, coordinates, earth, ecliptic, moon};
use crate::date::date::Date;
use crate::date::jd::JD;
use crate::moon::position::{geocentric_latitude, geocentric_longitude};
use crate::refraction::refraction_for_true_altitude;
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

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
/// date: Date to compute the rise time for
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
/// target_altitude: altitude of moon above horizon, in degrees [0, 90]
pub(crate) fn rise(
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    pressure: f64,
    temperature: f64,
) -> OutputKind {
    calculate_rise_set_transit(
        InputKind::Rise,
        date,
        longitude_observer,
        latitude_observer,
        pressure,
        temperature,
    )
}

/// Compute the time the moon sets
/// In:
/// date: Date to compute the set time for
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
/// target_altitude: altitude of moon above horizon, in degrees [0, 90]
pub(crate) fn set(
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    pressure: f64,
    temperature: f64,
) -> OutputKind {
    calculate_rise_set_transit(
        InputKind::Set,
        date,
        longitude_observer,
        latitude_observer,
        pressure,
        temperature,
    )
}

/// Compute the time the moon transits (i.e. is in the meridian)
/// In:
/// date: Date to compute the transit time for
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
/// target_altitude: altitude of moon above horizon, in degrees [0, 90]
pub(crate) fn transit(
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    pressure: f64,
    temperature: f64,
) -> OutputKind {
    calculate_rise_set_transit(
        InputKind::Transit,
        date,
        longitude_observer,
        latitude_observer,
        pressure,
        temperature,
    )
}

fn target_altitude(
    jd: JD,
    altitude: Degrees,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    pressure: f64,
    temperature: f64,
) -> Degrees {
    // SS:Moon's horizontal  at 0 deg altitude (i.e. at the horizon)
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
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    pressure: f64,
    temperature: f64,
) -> OutputKind {
    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();

    // SS: initial time is noon
    let midday = Date::new(date.year, date.month, date.day.trunc() + 0.5);
    let mut prev_jd = JD::from_date(midday);

    let target_altitude = target_altitude(
        prev_jd,
        Degrees::new(0.0),
        longitude_observer,
        latitude_observer,
        pressure,
        temperature,
    );
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

    OutputKind::Time(prev_jd)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::date::date::Date;

    use super::*;

    #[test]
    fn rise_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.0);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        // Act
        match rise(date, longitude_observer, latitude_observer, 1013.0, 10.0) {
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
        let date = Date::new(2000, 4, 22.0);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        // Act
        match rise(date, longitude_observer, latitude_observer, 1013.0, 10.0) {
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

                unreachable!()
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
        let date = Date::new(2000, 3, 23.0);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        // Act
        match set(date, longitude_observer, latitude_observer, 1013.0, 10.0) {
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
    fn transit_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.0);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);
        let latitude_observer = Degrees::new(48.1);

        // Act
        match transit(date, longitude_observer, latitude_observer, 1013.0, 10.0) {
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
