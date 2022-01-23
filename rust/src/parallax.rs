//! Calculate corrections for parallax.

use crate::time::{apparent_siderial_time, hour_angle, siderial_time_local};
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

const AU: f64 = 149_597_870.700;

/// Calculate the corrections needed to convert from geographical observer
/// latitude to the geocentric observer latitude.
/// Meeus, page 82, chapter 11
/// In: geographical latitude of the observer, in degrees [-90, 90)
/// height: Height of observer above sea level, in meters
/// Out: (rho * sin phi_p, rho * cos phi_p)
fn rho_phi_prime(latitude_geographical: Degrees, height: f64) -> (f64, f64) {
    let phi_p_radians = Radians::from(latitude_geographical);

    const B_OVER_A: f64 = 0.996_647_19;

    let u = (B_OVER_A * phi_p_radians.0.tan()).atan();

    let rho_sin_phi_p = B_OVER_A * u.sin() + height / (6_378_140.0) * phi_p_radians.0.sin();
    let rho_cos_phi_p = u.cos() + height / (6_378_140.0) * phi_p_radians.0.cos();

    (rho_sin_phi_p, rho_cos_phi_p)
}

/// Given the geocentric equatorial coordinates, calculate the topocentric ones
/// (i.e. the ones with the observer at the center of the coordinate system).
/// They are different, because the Earth is not a perfect sphere, but rather
/// a rotation ellipsoid due to flattening at the poles.
/// In:
/// ra: Right ascension, geocentric, apparent, in degrees [0, 360)
/// decl: Declination, geocentric, apparent, in degrees [-90, 90)
/// longitude: observer's longitude, in degrees [-80, 180)
/// latitude: Observer's geocentric latitude, in degrees [-90, 90)
/// height: observer's height above sea level, in meters
/// distance: distance of object to Earth, in km
/// jd: Julian Day
/// Out:
/// right ascension, topocentric, in dgrees [0, 360)
/// declination, topocentric, in degrees [-90, 90)
fn equatorial_2_topocentric(
    ra: Degrees,
    decl: Degrees,
    longitude: Degrees,
    latitude: Degrees,
    height: f64,
    distance: f64,
    jd: f64,
) -> (Degrees, Degrees) {
    let (rho_sin_p, rho_cos_p) = rho_phi_prime(latitude, height);

    let delta = distance / AU;
    let sin_pi = Radians::from(Degrees::from(ArcSec::new(8.794))).0.sin() / delta;

    // SS: calculate local hour angle
    let siderial_time_greenwich = apparent_siderial_time(jd);
    let siderial_time_local = siderial_time_local(siderial_time_greenwich, longitude);
    let hour_angle = hour_angle(siderial_time_local, ra);
    let hour_angle_radians = Radians::from(hour_angle);

    let ra_radians = Radians::from(ra);
    let decl_radians = Radians::from(decl);

    let delta_ra = (-rho_cos_p * sin_pi * hour_angle_radians.0.sin())
        .atan2(decl_radians.0.cos() - rho_cos_p * sin_pi * hour_angle_radians.0.cos());
    let ra_topocentric = ra_radians + Radians::new(delta_ra);

    let decl_topocentric = ((decl_radians.0.sin() - rho_sin_p * sin_pi) * delta_ra.cos())
        .atan2(decl_radians.0.cos() - rho_cos_p * sin_pi * hour_angle_radians.0.cos());
    let decl_topocentric = Radians::new(decl_topocentric);

    (
        Degrees::from(ra_topocentric),
        Degrees::from(decl_topocentric),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jd;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn rho_phi_p_test() {
        // Meeus, page 82, example 11.a

        // Arrange
        let palomar_latitude = Degrees::from_dms(33, 21, 22.0);
        let palomar_height = 1706.0;

        // Act
        let (rho_sin_p, rho_cos_p) = rho_phi_prime(palomar_latitude, palomar_height);

        // Assert
        assert_approx_eq!(0.546_861, rho_sin_p, 0.000_001);
        assert_approx_eq!(0.836_339, rho_cos_p, 0.000_001);
    }

    #[test]
    fn equatorial_to_topocentric_test() {
        // SS: Meeus, page 280, example 40.a

        // Act
        let jd = jd::from_date_hms(2003, 8, 28, 3, 17, 0.0);

        // SS: Mount Palomar
        let longitude_observer = Degrees::from_hms(7, 47, 27.0);
        let latitude_observer = Degrees::from_dms(33, 21, 22.0);
        let palomar_height_above_sea = 1706.0;

        let ra_geocentric_mars = Degrees::from_hms(22, 38, 7.25);
        let decl_geocentric_mars = Degrees::from_dms(-15, 46, 15.9);
        let distance_mars = 0.37276 * AU;

        let (ra_topocentric_mars, decl_topocentric_mars) = equatorial_2_topocentric(
            ra_geocentric_mars,
            decl_geocentric_mars,
            longitude_observer,
            latitude_observer,
            palomar_height_above_sea,
            distance_mars,
            jd,
        );

        // Assert
        assert_approx_eq!(
            Degrees::from_hms(22, 38, 8.54).0,
            ra_topocentric_mars.0,
            0.000_1
        );
        assert_approx_eq!(
            Degrees::from_dms(-15, 46, 30.0).0,
            decl_topocentric_mars.0,
            0.000_1
        );
    }
}
