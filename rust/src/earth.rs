//! Earth related calculations
use crate::{ecliptic, jd, util};

/// Calculate Earth's eccentricity, eq (47.6).
/// In: Julian day in dynamical time
pub fn eccentricity(jd: f64) -> f64 {
    let t = jd::centuries_from_epoch_j2000(jd);
    let t2 = t * t;

    1.0 - 0.002516 * t - 0.0000074 * t2
}

/// Convert from ecliptical coordinates (longitude, latitude) to
/// equatorial coordinates (right ascension, declination).
/// In: Julian day
/// longitude: Longitude in degrees [0, 360)
/// latitude: Latitude in degrees [0, 360)
/// Out: right ascension in degrees [0, 360)
/// declination in degrees [0, 360)
pub fn ecliptical_to_equatorial(jd: f64, longitude: f64, latitude: f64) -> (f64, f64) {
    let true_obliquity = ecliptic::true_obliquity(jd);
    let true_obliquity_radians = util::to_radians(true_obliquity);

    let longitude_radians = util::to_radians(longitude);
    let latitude_radians = util::to_radians(latitude);

    let ra_argument_x = longitude_radians.sin() * true_obliquity_radians.cos()
        - latitude_radians.tan() * true_obliquity_radians.sin();
    let ra_radians = ra_argument_x.atan2(longitude_radians.cos());

    let dec_argument_x = latitude_radians.sin() * true_obliquity_radians.cos()
        + latitude_radians.cos() * true_obliquity_radians.sin() * longitude_radians.sin();
    let dec_radians = dec_argument_x.asin();

    (util::to_degrees(ra_radians), util::to_degrees(dec_radians))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn eccentricity_test() {
        // Arrange

        // SS: April 12th, 1992, 0h TD
        let jd = 2_448_724.5;

        // Act
        let e = eccentricity(jd);

        // Assert
        assert_approx_eq!(1.000194, e, 0.000001)
    }

    #[test]
    pub fn ecliptical_to_equatorial_test() {
        // Arrange
        let jd = jd::from_date(1992, 4, 12, 0.0);
        let longitude = 133.162655;
        let latitude = -3.229126;

        // Act
        let (ra, dec) = ecliptical_to_equatorial(jd, longitude, latitude);

        // Assert
        assert_approx_eq!(8.9789280347415126, util::degrees_to_hours(ra), 0.000_001);
        assert_approx_eq!(13.769657226951539, dec, 0.000_001);
    }
}
