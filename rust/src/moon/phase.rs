//! Phase of the moon
use crate::earth::ecliptical_to_equatorial;
use crate::sun::position::{
    apparent_geometric_latitude, apparent_geometric_longitude, distance_earth_sun,
    geocentric_ecliptical_latitude,
};
use crate::{jd, moon, util};

const SYNODIC_MONTH: f64 = 29.53058868;
const SYNODIC_MONTH_OVER_2: f64 = SYNODIC_MONTH / 2.0;

/// Calculate the phase angle or age of the moon.
/// Meeus, chapter 48, eq. (48.1) or Duffett-Smith and Zwart, chapter 67, page 171
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
    let tani = (r * psi.sin()) / (delta - r * psi.cos());
    let phase_angle = (r * psi.sin()).atan2(delta - r * psi.cos());
    phase_angle
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
        // SS: 2021 Nov. 29, 12:33am TD
        let jd = jd::from_date(2021, 11, 29, 0.525);
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let phase_angle = phase_angle(jd);

        // Assert
        assert_approx_eq!(63.9091644374556, phase_angle, 0.000_001)
    }

    #[test]
    fn fraction_illuminated_test() {
        // SS: 2021 Nov. 29, 12:33am TD
        let jd = jd::from_date(2021, 11, 29, 0.525);

        // Act
        let fraction_illuminated = fraction_illuminated(jd);

        // Assert
        assert_approx_eq!(1.0 - 0.7198977625352061, fraction_illuminated, 0.000_001)
    }
}
