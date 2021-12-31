//! Phase of the moon
use crate::earth::ecliptical_to_equatorial;
use crate::sun::position::{
    apparent_geometric_latitude, apparent_geometric_longitude, distance_earth_sun,
};
use crate::util::{Degrees, Radians};
use crate::{jd, moon, util};

/// Calculate the phase angle or age of the moon.
/// Meeus, chapter 48, eq. (48.1) or Duffett-Smith and Zwart, chapter 67, page 171
/// In: Julian day
/// Out: Phase angle, in degrees [0, 360)
pub fn phase_angle(jd: f64) -> Degrees {
    // SS: position of the moon, from Earth
    let longitude = moon::position::geocentric_longitude(jd);
    let latitude = moon::position::geocentric_latitude(jd);
    let delta = moon::position::distance_from_earth(jd);
    let (ra_moon, dec_moon) = ecliptical_to_equatorial(jd, longitude, latitude);
    let (ra_moon, dec_moon) = (Radians::from(ra_moon), Radians::from(dec_moon));

    // SS: position of the sun, from Earth
    let longitude = apparent_geometric_longitude(jd);
    let latitude = apparent_geometric_latitude(jd);
    let r = distance_earth_sun(jd);
    let (ra_sun, dec_sun) = ecliptical_to_equatorial(jd, longitude, latitude);
    let (ra_sun, dec_sun) = (Radians::from(ra_sun), Radians::from(dec_sun));

    // SS: geocentric elongation of the moon from the sun
    // Meeus, eq. (48.2)
    let psi = (dec_sun.0.sin() * dec_moon.0.sin()
        + dec_sun.0.cos() * dec_moon.0.cos() * (ra_sun.0 - ra_moon.0).cos())
    .acos();

    // SS: phase angle
    let phase_angle = (r * psi.sin()).atan2(delta - r * psi.cos());
    util::map_to_0_to_360(Degrees::from(Radians::new(phase_angle)))
}

/// Calculate the phase angle or age of the moon.
/// Duffett-Smith and Zwart, chapter 67, page 171
/// In: Julian day
/// Out: Phase angle, in degrees [0, 360)
pub fn phase_angle_360(jd: f64) -> Degrees {
    // SS: position of the moon, from Earth
    let longitude_moon = moon::position::geocentric_longitude(jd);

    // SS: position of the sun, from Earth
    let longitude_sun = apparent_geometric_longitude(jd);

    let phase_angle = util::map_to_0_to_360(longitude_moon - longitude_sun);
    phase_angle
}

/// Textual description of the moon's phase
/// In: Julian day
/// Out: Textual description
pub fn phase_description(jd: f64) -> &'static str {
    let illuminated_fraction = fraction_illuminated(jd);

    const SECTION: f64 = 1.0 / 12.0;

    let desc = if illuminated_fraction <= SECTION {
        "New Moon"
    } else if illuminated_fraction <= 2.0 * SECTION {
        "Waxing Crescent"
    } else if illuminated_fraction >= 45.0 - SECTION && illuminated_fraction < 45.0 + SECTION {
        "First Quarter"
    } else if illuminated_fraction >= 45.0 + SECTION && illuminated_fraction < 90.0 - SECTION {
        "Waning Crescent"
    } else if illuminated_fraction >= 90.0 - SECTION && illuminated_fraction < 90.0 + SECTION {
        "Full Moon"
    } else if illuminated_fraction >= 90.0 + SECTION && illuminated_fraction < 135.0 - SECTION {
        "Waxing Crescent"
    } else if illuminated_fraction >= 135.0 - SECTION && illuminated_fraction < 135.0 + SECTION {
        "First Quarter"
    } else if illuminated_fraction >= 135.0 + SECTION && illuminated_fraction < 180.0 - SECTION {
        "Waxing Gibbous"
    } else {
        "Waning Crescent"
    };

    desc
}

pub fn fraction_illuminated(jd: f64) -> f64 {
    let phase_angle = Radians::from(phase_angle(jd));
    (1.0 + phase_angle.0.cos()) / 2.0
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
        assert_approx_eq!(69.07565471001595, phase_angle.0, 0.000_001)
    }

    #[test]
    fn fraction_illuminated_test1() {
        // Arrange
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let fraction_illuminated = fraction_illuminated(jd);

        // Assert
        assert_approx_eq!(0.6785674578465415, fraction_illuminated, 0.000_001)
    }

    #[test]
    fn fraction_illuminated_test2() {
        // Arrange

        // SS: Dec. 4th, 2021, 12:26PM local Denver time
        let jd = 2_459_553.3;

        // Act
        let percent_illuminated = fraction_illuminated(jd) * 100.0;

        // Assert
        assert_approx_eq!(0.373, percent_illuminated, 0.001)
    }

    #[test]
    fn phase_description_test_1() {
        // Arrange

        // SS: Dec. 4th, 2021, 12:26PM local Denver time
        let jd = 2_459_553.3;

        // Act
        let phase_desc = phase_description(jd);

        // Assert
        assert_eq!("New Moon", phase_desc)
    }
    //
    // #[test]
    // fn phase_description_test_7() {
    //     // Arrange
    //
    //     // SS: Dec. 4th, 2021, 12:26PM local Denver time
    //     let mut jd = 2_459_553.3;
    //
    //     for i in 0..30 {
    //         // Act
    //         let phase_desc = phase_angle2(jd + i as f64);
    //
    //         println!("{}", phase_desc);
    //     }
    //
    //     // Assert
    //     //        assert_eq!("New Moon", phase_desc)
    // }

    #[test]
    fn phase_description_test_2() {
        // Arrange

        // SS: Dec. 8th, 2021, 12:37PM local Denver time
        let jd = 2_459_557.338747;

        // Act
        let phase_desc = phase_description(jd);

        // Assert
        assert_eq!("Waxing Crescent", phase_desc)
    }
}
