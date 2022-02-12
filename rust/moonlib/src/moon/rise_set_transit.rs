//! Calculate rise, set and transit times for the moon

use crate::date::jd::JD;
use crate::{constants, coordinates, earth, ecliptic};
use crate::moon::position::{geocentric_latitude, geocentric_longitude};
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

pub enum Kind {
    Time(JD),
    NeverRises,
    NeverSets
}

/// Compute the time the moon rises
/// In:
/// jd: Julian Day to compute the rise time for
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
/// target_altitude: altitude of moon above horizon, in degrees [0, 90]
pub(crate) fn rise(jd: JD, longitude_observer: Degrees, latitude_observer: Degrees, target_altitude: Degrees) -> Kind {

    let mut jd2 = jd;

    let target_altitude_radians = Radians::from(target_altitude);
    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();
    let sin_h0 = target_altitude_radians.0.sin();

    let mut iter = 0;
    const MAX_ITER: u8 = 10;

    loop {
        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(jd2);
        let latitude = geocentric_latitude(jd2);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(jd2);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        let sin_decl = Radians::from(decl).0.sin();
        let cos_decl = Radians::from(decl).0.cos();
        let cos_hour_angle = (sin_h0 - sin_latitude_observer * sin_decl)/(cos_latitude_observer * cos_decl);

        let hour_angle;

        if cos_hour_angle < -1.0 {
            return Kind::NeverRises;
        }
        else if cos_hour_angle > 1.0 {
            return Kind::NeverSets;
        }
        else {
            hour_angle = Radians::new(cos_hour_angle.acos());
        }

        // SS: calculate time correction from our angle
        let siderial_time_apparent_greenwich = earth::apparent_siderial_time(jd2);
        let siderial_time_local = earth::local_siderial_time(siderial_time_apparent_greenwich, longitude_observer);

        // SS: calculate hour angle at time jd2
        let hour_angle2 = siderial_time_local - ra;

        let delta_hour_angle = Radians::from(hour_angle2) - hour_angle;

        // SS: calculate time correction
        let delta_siderial_t = delta_hour_angle;

        let delta_t_radians = delta_siderial_t * Radians::new(constants::SIDERIAL_TO_SOLAR_TIME);
        let delta_t = Degrees::from(delta_t_radians).to_hours();

        if delta_t < 0.008 || iter > MAX_ITER {
            break;
        }

        iter += 1;

        jd2.add_hours(delta_t);
    }

    Kind::Time(jd2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use crate::date::date::Date;

    #[test]
    fn rise_test_1() {
        // Arrange
        let jd = JD::from_date(Date::new(2000, 3, 23.0));

        // SS: 16 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-16.0);

        let latitude_observer = Degrees::new(65.0);
        let target_altitude = Degrees::new(constants::MOON_SET_HEIGHT);

        // Act
        let k = rise(jd, longitude_observer, latitude_observer, target_altitude);

        // Assert
//        assert_approx_eq!(-180.0 + (d.0 - 180.0), angle.0, 0.000_001)
    }
}
