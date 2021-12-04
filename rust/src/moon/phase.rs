//! Phase of the moon
use crate::earth::ecliptical_to_equatorial;
use crate::sun::position::{
    apparent_geometric_latitude, apparent_geometric_longitude, distance_earth_sun,
    geocentric_ecliptical_latitude,
};
use crate::{jd, moon, util};

/// Calculate the phase angle or age of the moon.
/// Meeus, chapter 48, eq. (48.1) or Duffett-Smith and Zwart, chapter 67, page 171
/// In: Julian day
/// Out: Phase angle, in degrees [0, 360)
pub fn phase_angle(jd: f64) -> f64 {
    // SS: position of the moon, from Earth
    let longitude = moon::position::longitude(jd);
    let latitude = moon::position::latitude(jd);
    let delta = moon::position::distance_from_earth(jd);
    let (ra_moon, dec_moon) = ecliptical_to_equatorial(jd, longitude, latitude);
    let (ra_moon, dec_moon) = (util::to_radians(ra_moon), util::to_radians(dec_moon));

    // SS: position of the sun, from Earth
    let longitude = apparent_geometric_longitude(jd);
    let latitude = apparent_geometric_latitude(jd);
    let r = distance_earth_sun(jd);
    let (ra_sun, dec_sun) = ecliptical_to_equatorial(jd, longitude, latitude);
    let (ra_sun, dec_sun) = (util::to_radians(ra_sun), util::to_radians(dec_sun));

    // SS: geocentric elongation of the moon from the sun
    // Meeus, eq. (48.2)
    let psi = (dec_sun.sin() * dec_moon.sin()
        + dec_sun.cos() * dec_moon.cos() * (ra_sun - ra_moon).cos())
    .acos();

    // SS: phase angle
    let phase_angle = (r * psi.sin()).atan2(delta - r * psi.cos());
    util::to_degrees(phase_angle)
}

pub fn fraction_illuminated(jd: f64) -> f64 {
    let phase_angle = util::to_radians(phase_angle(jd));
    (1.0 + phase_angle.cos()) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn phase_angle_test() {
        // Arrange
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let phase_angle = phase_angle(jd);

        // Assert
        assert_approx_eq!(69.07565471001595, phase_angle, 0.000_001)
    }

    #[test]
    fn fraction_illuminated_test() {
        // Arrange
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let fraction_illuminated = fraction_illuminated(jd);

        // Assert
        assert_approx_eq!(0.6785674578465415, fraction_illuminated, 0.000_001)
    }
}
