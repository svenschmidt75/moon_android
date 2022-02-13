//! Calculate rise, set and transit times for the moon

use crate::date::date::Date;
use crate::date::jd::JD;
use crate::moon::position::{geocentric_latitude, geocentric_longitude};
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;
use crate::{constants, coordinates, earth, ecliptic};

pub enum Kind {
    Time(JD),
    NeverRises,
    NeverSets,
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
) -> Kind {
    // SS: initial time is noon
    let midday = Date::new(date.year, date.month, date.day.trunc() + 0.5);
    let base_jd = JD::from_date(midday);
    let mut offset_jd = JD::new(0.0);

    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();

    let target_altitude_radians = Radians::from(target_altitude);
    let sin_h0 = target_altitude_radians.0.sin();

    let mut iter = 0;
    const MAX_ITER: u8 = 10;

    loop {
        print!("Iteration {iter}: ");

        let jd = base_jd + offset_jd;

        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(base_jd + offset_jd);
        let latitude = geocentric_latitude(base_jd + offset_jd);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(base_jd + offset_jd);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        let ra_hours = ra.to_hms();

        let decl_radians = Radians::from(decl);
        let sin_decl = decl_radians.0.sin();
        let cos_decl = decl_radians.0.cos();
        let cos_hour_angle =
            (sin_h0 - sin_latitude_observer * sin_decl) / (cos_latitude_observer * cos_decl);

        let hour_angle;

        if cos_hour_angle < -1.0 {
            return Kind::NeverRises;
        } else if cos_hour_angle > 1.0 {
            return Kind::NeverSets;
        } else {
            hour_angle = Degrees::from(Radians::new(cos_hour_angle.acos()));
        }

        // SS: calculate time correction from our angle
        let theta0 = earth::apparent_siderial_time(base_jd + offset_jd);
        let theta = earth::local_siderial_time(theta0, longitude_observer);
        let theta_hours = theta.to_hms();

        // SS: calculate hour angle at time jd2
        let hour_angle2 = (theta - ra).map_neg180_to_180();
        let hour_angle_hours = hour_angle2.to_hms();

        let delta_hour_angle = (hour_angle2 + hour_angle).map_neg180_to_180();

        // SS: convert degrees to time units
        let delta_t = delta_hour_angle.to_hours() * constants::SIDERIAL_TO_SOLAR_TIME;

        let delta_t_hours = Degrees::new(delta_hour_angle.0 * constants::SIDERIAL_TO_SOLAR_TIME).to_hms();

        println!(
            "tau1 {:.2} -- tau2 {:.2} -- delta tau: {:.2} -- delta t {:.2}",
            hour_angle.0, hour_angle2.0, delta_hour_angle.0, delta_t
        );

        if delta_t.abs() < 0.008 || iter > MAX_ITER {
            break;
        }

        iter += 1;

        offset_jd.add_hours(delta_t);
    }

    Kind::Time(base_jd + offset_jd)
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

        // Assert
        //        assert_approx_eq!(-180.0 + (d.0 - 180.0), angle.0, 0.000_001)
    }
}
