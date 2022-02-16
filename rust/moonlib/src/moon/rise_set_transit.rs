//! Calculate rise, set and transit times for the moon

use crate::date::date::Date;
use crate::date::jd::JD;
use crate::moon::position::{geocentric_latitude, geocentric_longitude};
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;
use crate::{constants, coordinates, earth, ecliptic};

pub(crate) enum OutputKind {
    Time(JD),
    NeverRises,
    NeverSets,
}

enum InputKind {
    Rise,
    Set,
    Transit
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
    target_altitude: Degrees,
) -> OutputKind {
    calculate_rise_set_transit(InputKind::Rise, date, longitude_observer, latitude_observer, target_altitude)
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
    target_altitude: Degrees,
) -> OutputKind {
    calculate_rise_set_transit(InputKind::Set, date, longitude_observer, latitude_observer, target_altitude)
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
    target_altitude: Degrees,
) -> OutputKind {
    calculate_rise_set_transit(InputKind::Transit, date, longitude_observer, latitude_observer, target_altitude)
}

fn calculate_rise_set_transit(
    kind: InputKind,
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    target_altitude: Degrees,
) -> OutputKind {
    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();

    // SS: initial time is noon
    let midday = Date::new(date.year, date.month, date.day.trunc() + 0.5);
    let mut prev_jd = JD::from_date(midday);

    let target_altitude_radians = Radians::from(target_altitude);
    let sin_h0 = target_altitude_radians.0.sin();

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
        let cos_hour_angle = (sin_h0 - sin_latitude_observer * sin_decl) / (cos_latitude_observer * cos_decl);

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
    use super::*;
    use crate::date::date::Date;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn rise_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.0);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);

        let latitude_observer = Degrees::new(48.1);
        let target_altitude = Degrees::new(constants::MOON_SET_HEIGHT);

        // Act
        let k = rise(date, longitude_observer, latitude_observer, target_altitude);

        if let OutputKind::Time(jd) = k {
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
        }

        // Assert

        //        assert_approx_eq!(-180.0 + (d.0 - 180.0), angle.0, 0.000_001)
    }
}
