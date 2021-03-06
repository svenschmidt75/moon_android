//! Coordinate transformations

use crate::date::jd::JD;
use crate::util::{degrees::Degrees, radians::Radians};
use crate::{constants, earth, parallax, util};

/// Convert ecliptical to equatorial coordinates.
/// Meeus, page 93, chapter 13
/// In:
/// lambda: longitude, in degrees [0, 360)
/// beta: latitude, in degrees [0, 360)
/// eps: obliquity of the eclipse. Use true
/// obliquity for apparent right ascension and
/// declination, in degrees
/// Out:
/// right ascension, in degrees [0, 360)
/// declination, in degrees [-90, 90)
pub(crate) fn ecliptical_2_equatorial(
    lambda: Degrees,
    beta: Degrees,
    eps: Degrees,
) -> (Degrees, Degrees) {
    let lambda_radians = Radians::from(lambda);
    let beta_radians = Radians::from(beta);
    let eps_radians = Radians::from(eps);

    let alpha = (lambda_radians.0.sin() * eps_radians.0.cos()
        - beta_radians.0.tan() * eps_radians.0.sin())
    .atan2(lambda_radians.0.cos());
    let delta = (beta_radians.0.sin() * eps_radians.0.cos()
        + beta_radians.0.cos() * eps_radians.0.sin() * lambda_radians.0.sin())
    .asin();

    (
        Degrees::from(Radians::new(alpha)).map_to_0_to_360(),
        Degrees::from(Radians::new(delta)).map_to_neg90_to_90(),
    )
}

/// Calculate horizontal from equatorial coordinates. Note that A is measured
/// eastward from the North, whereas in Meeus, it is measures westward from
/// the South!
/// In:
/// declination, in degrees [-90, 90)
/// hour_angle, in degrees [0, 360)
/// observer's latitude, [-90, 90)
/// Out:
/// Azimuth, measured from North, increasing to the East, in degrees [0, 360)
/// Altitude: in degrees [-90, 90)
pub(crate) fn equatorial_2_horizontal(
    decl: Degrees,
    hour_angle: Degrees,
    latitude_observer: Degrees,
) -> (Degrees, Degrees) {
    let decl_radians = Radians::from(decl);
    let hour_angle_radians = Radians::from(hour_angle);
    let latitude_observer_radians = Radians::from(latitude_observer);

    let altitude_arg = latitude_observer_radians.0.sin() * decl_radians.0.sin()
        + latitude_observer_radians.0.cos() * decl_radians.0.cos() * hour_angle_radians.0.cos();
    let altitude = altitude_arg.asin();

    let mut azimuth = ((decl_radians.0.sin()
        - latitude_observer_radians.0.sin() * altitude_arg.sin())
        / (latitude_observer_radians.0.cos() * altitude_arg.cos()))
    .acos();

    // SS: The range of acos is [0, 180), but azimuth should be in the range [0, 360).
    // Hence, we need a way to extent the result of acos. When the hour angle is positive,
    // then so is it's sin. In this case, A should be in the western hemisphere, i.e. we
    // extent A by subtracting 360 (North) from it.
    let sinH = hour_angle_radians.0.sin();
    if sinH > 0.0 {
        azimuth = 2.0 * std::f64::consts::PI - azimuth;
    }

    (
        Degrees::from(Radians::new(azimuth)).map_to_0_to_360(),
        Degrees::from(Radians::new(altitude)),
    )
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
pub(crate) fn equatorial_2_topocentric(
    ra: Degrees,
    decl: Degrees,
    longitude: Degrees,
    latitude: Degrees,
    height: f64,
    distance: f64,
    jd: JD,
) -> (Degrees, Degrees) {
    let (rho_sin_p, rho_cos_p) = parallax::rho_phi_prime(latitude, height);

    let delta = distance / constants::AU;
    let sin_pi = Radians::from(Degrees::from(util::arcsec::ArcSec::new(8.794)))
        .0
        .sin()
        / delta;

    // SS: calculate local hour angle
    let siderial_time_greenwich = earth::apparent_siderial_time(jd);
    let siderial_time_local = earth::local_siderial_time(siderial_time_greenwich, longitude);
    let hour_angle = earth::hour_angle(siderial_time_local, ra);
    let hour_angle_radians = Radians::from(hour_angle);

    let ra_radians = Radians::from(ra);
    let decl_radians = Radians::from(decl);

    // SS: eq (40.2)
    let delta_ra = (-rho_cos_p * sin_pi * hour_angle_radians.0.sin())
        .atan2(decl_radians.0.cos() - rho_cos_p * sin_pi * hour_angle_radians.0.cos());
    let ra_topocentric = ra_radians + Radians::new(delta_ra);

    // SS: eq (40.3)
    let decl_topocentric = ((decl_radians.0.sin() - rho_sin_p * sin_pi) * delta_ra.cos())
        .atan2(decl_radians.0.cos() - rho_cos_p * sin_pi * hour_angle_radians.0.cos());
    let decl_topocentric = Radians::new(decl_topocentric);

    (
        Degrees::from(ra_topocentric).map_to_0_to_360(),
        Degrees::from(decl_topocentric).map_to_neg90_to_90(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn example_13a() {
        // Arrange
        let longitude = Degrees::new(113.215630);
        let latitude = Degrees::new(6.684170);
        let eps = Degrees::new(23.4392911);

        // Act
        let (ra, decl) = ecliptical_2_equatorial(longitude, latitude, eps);

        // Assert
        let (h, m, s) = ra.to_hms();
        assert_eq!(7, h);
        assert_eq!(45, m);
        assert_approx_eq!(18.946, s, 0.001);

        let (d, m, s) = decl.to_dms();
        assert_eq!(28, d);
        assert_eq!(1, m);
        assert_approx_eq!(34.26, s, 0.01);
    }

    #[test]
    fn equatorial_2_horizontal_test_1() {
        // Meeus, page 96, example 13.b

        // Arrange
        let declination = Degrees::from_dms(-6, 43, 11.61);
        let hour_angle = Degrees::new(64.352133);
        let latitude_observer = Degrees::from_dms(38, 55, 17.0);

        // Act
        let (azimuth, altitude) =
            equatorial_2_horizontal(declination, hour_angle, latitude_observer);

        // Assert
        // SS: +180 because 68.0337 deg is the result when A is measured westward
        // from the South.
        assert_approx_eq!(180.0 + 68.0337, azimuth.0, 0.2);
        assert_approx_eq!(15.1249, altitude.0, 0.000_1);
    }

    #[test]
    fn equatorial_2_horizontal_test_2() {
        // J.L. Lawrence, Celestial Calculations, 2018, page 90

        // Arrange
        let declination = Degrees::from_dms(-0, 30, 30.0);
        let hour_angle = Degrees::from_hms(16, 29, 45.0);
        let latitude_observer = Degrees::from_dms(25, 0, 0.0);

        // Act
        let (azimuth, altitude) =
            equatorial_2_horizontal(declination, hour_angle, latitude_observer);

        // Assert
        assert_approx_eq!(Degrees::from_dms(80, 31, 31.0).0, azimuth.0, 1.0);
        assert_approx_eq!(Degrees::from_dms(-20, 34, 40.0).0, altitude.0, 1.0);
    }
}
