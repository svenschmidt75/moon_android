//! Coordinate transformations

use crate::util::{degrees::Degrees, radians::Radians};

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
pub(crate) fn ecliptic_2_equatorial(lambda: Degrees, beta: Degrees, eps: Degrees) -> (Degrees, Degrees) {
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
        Degrees::from(Radians::new(alpha)),
        Degrees::from(Radians::new(delta)),
    )
}

/// Calculate horizontal from equatorial coordinates.
/// In:
/// declination, in degrees [-90, 90)
/// hour_angle, in degrees [0, 360)
/// observer's latitude, [-90, 90)
fn equatorial_2_horizontal(
    decl: Degrees,
    hour_angle: Degrees,
    latitude_observer: Degrees,
) -> (Degrees, Degrees) {
    let decl_radians = Radians::from(decl);
    let hour_angle_radians = Radians::from(hour_angle);
    let latitude_observer_radians = Radians::from(latitude_observer);

    let azimuth = hour_angle_radians.0.sin().atan2(
        hour_angle_radians.0.cos() * latitude_observer_radians.0.sin()
            - decl_radians.0.tan() * latitude_observer_radians.0.cos(),
    );
    let altitude = (latitude_observer_radians.0.sin() * decl_radians.0.sin()
        + latitude_observer_radians.0.cos() * decl_radians.0.cos() * hour_angle_radians.0.cos())
    .asin();

    (
        Degrees::from(Radians::new(azimuth)),
        Degrees::from(Radians::new(altitude)),
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
        let (ra, decl) = ecliptic_2_equatorial(longitude, latitude, eps);

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
    fn equatorial_2_horizontal_test() {
        // Meeus, page 96, example 13.b

        // Arrange
        let declination = Degrees::from_dms(-6, 43, 11.61);
        let hour_angle = Degrees::new(64.352133);
        let latitude_observer = Degrees::from_dms(38, 55, 17.0);

        // Act
        let (azimuth, altitude) =
            equatorial_2_horizontal(declination, hour_angle, latitude_observer);

        // Assert
        assert_approx_eq!(68.0337, azimuth.0, 0.000_1);
        assert_approx_eq!(15.1249, altitude.0, 0.000_1);
    }
}