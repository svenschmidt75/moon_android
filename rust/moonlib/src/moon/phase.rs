//! Phase of the moon
use crate::date::jd::JD;
use crate::sun::position::{
    apparent_geocentric_longitude, apparent_geometric_latitude, distance_earth_sun,
};
use crate::util::{degrees::Degrees, radians::Radians};
use crate::{constants, coordinates, ecliptic, moon};

/// Calculate the phase angle or age of the moon.
/// Meeus, chapter 48, eq. (48.1) or Duffett-Smith and Zwart, chapter 67, page 171
/// In: Julian day
/// Out: Phase angle, in degrees [0, 360)
pub fn phase_angle(jd: JD) -> Degrees {
    // SS: position of the moon, from Earth
    let longitude = moon::position::geocentric_longitude(jd);
    let latitude = moon::position::geocentric_latitude(jd);
    let true_obliquity = ecliptic::true_obliquity(jd);
    let (ra_moon, dec_moon) =
        coordinates::ecliptical_2_equatorial(longitude, latitude, true_obliquity);
    let (ra_moon, dec_moon) = (Radians::from(ra_moon), Radians::from(dec_moon));

    // SS: position of the sun, from Earth
    let longitude = apparent_geocentric_longitude(jd);
    let latitude = apparent_geometric_latitude(jd);
    let r = distance_earth_sun(jd);
    let (ra_sun, dec_sun) =
        coordinates::ecliptical_2_equatorial(longitude, latitude, true_obliquity);
    let (ra_sun, dec_sun) = (Radians::from(ra_sun), Radians::from(dec_sun));

    // SS: geocentric elongation of the moon from the sun
    // Meeus, eq. (48.2)
    let psi = (dec_sun.0.sin() * dec_moon.0.sin()
        + dec_sun.0.cos() * dec_moon.0.cos() * (ra_sun.0 - ra_moon.0).cos())
    .acos();

    let delta = moon::position::distance_from_earth(jd);

    // SS: phase angle
    let phase_angle = (r * psi.sin()).atan2(delta - r * psi.cos());
    Degrees::from(Radians::new(phase_angle)).map_to_0_to_360()
}

/// Calculate the phase angle or age of the moon.
/// Duffett-Smith and Zwart, chapter 67, page 171
/// In: Julian day
/// Out: Phase angle, in degrees [0, 360)
pub fn phase_angle_360(jd: JD) -> Degrees {
    // SS: position of the moon, from Earth
    let longitude_moon = moon::position::geocentric_longitude(jd);

    // SS: position of the sun, from Earth
    let longitude_sun = apparent_geocentric_longitude(jd);

    (longitude_moon - longitude_sun).map_to_0_to_360()
}

/// Age of the moon phase in fractions of a day.
/// In: Julian day
/// Out: Phase age, in fractional days
pub(crate) fn phase_age(jd: JD) -> f64 {
    let phase_angle = phase_angle_360(jd);

    // SS: Avg. degrees the Moon advances in its orbit around the Earth
    // per day.
    phase_angle.0 / constants::MOON_DAY
}

/// Textual description of the moon's phase
/// In: Julian day
/// Out: Textual description
pub(crate) fn phase_description(jd: JD) -> &'static str {
    let phase_angle = phase_angle_360(jd).0;

    const SECTION: f64 = 360.0 / (2.0 * 8.0);

    if phase_angle < SECTION {
        "New Moon"
    } else if (SECTION..45.0 + SECTION).contains(&phase_angle) {
        "Waxing Crescent"
    } else if (45.0 + SECTION..90.0 + SECTION).contains(&phase_angle) {
        "First Quarter"
    } else if (90.0 + SECTION..180.0 - SECTION).contains(&phase_angle) {
        "Waxing Gibbous"
    } else if (180.0 - SECTION..180.0 + SECTION).contains(&phase_angle) {
        "Full Moon"
    } else if (180.0 + SECTION..270.0 - SECTION).contains(&phase_angle) {
        "Waning Gibbous"
    } else if (270.0 - SECTION..270.0 + SECTION).contains(&phase_angle) {
        "Last Quarter"
    } else {
        //if phase_angle >= 270.0 + SECTION && phase_angle < 180.0 + 45.0 - SECTION {
        "Waning Crescent"
    }
}

pub fn fraction_illuminated(jd: JD) -> f64 {
    let phase_angle = Radians::from(phase_angle(jd));
    (1.0 + phase_angle.0.cos()) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::date::Date;
    use crate::date::jd::JD;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn phase_angle_test_1() {
        // Arrange
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let phase_angle = phase_angle(jd);

        // Assert
        assert_approx_eq!(69.07565471001595, phase_angle.0, 0.000_001)
    }

    #[test]
    fn phase_angle_test_2() {
        // J.L. Lawrence, Celestial Calculations, 2018, page 180

        // Arrange
        let date = Date::new(2015, 1, 1.0);
        let jd = JD::from_date(date);

        // Act
        let phase_angle = phase_angle_360(jd);

        // Assert
        assert_approx_eq!(130.38, phase_angle.0, 0.1)
    }

    #[test]
    fn phase_age_test_1() {
        // Arrange
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let phase_age = phase_age(jd);

        // Assert
        assert_approx_eq!(9.091, phase_age, 0.001)
    }

    #[test]
    fn phase_age_test_2() {
        // Arrange
        let jd = JD::from_date(Date::new(2022, 2, 26.0));

        // Act
        let phase_age = phase_age(jd);

        // Assert
        assert_approx_eq!(24.37, phase_age, 0.01)
    }

    #[test]
    fn fraction_illuminated_test_1() {
        // Arrange
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let fraction_illuminated = fraction_illuminated(jd);

        // Assert
        assert_approx_eq!(0.6785674578465415, fraction_illuminated, 0.000_001)
    }

    #[test]
    fn fraction_illuminated_test_2() {
        // Arrange

        // SS: Dec. 4th, 2021, 12:26PM local Denver time
        let jd = JD::new(2_459_553.3);

        // Act
        let percent_illuminated = fraction_illuminated(jd) * 100.0;

        // Assert
        assert_approx_eq!(0.373, percent_illuminated, 0.001)
    }

    #[test]
    fn fraction_illuminated_test_3() {
        // Arrange

        // SS: Dec. 30th, 2021, 9:30PM local Denver time
        let jd = JD::new(2_459_580.187);

        // Act
        let percent_illuminated = fraction_illuminated(jd) * 100.0;

        // Assert
        assert_approx_eq!(6.4943, percent_illuminated, 0.001)
    }

    #[test]
    fn fraction_illuminated_test_4() {
        // J.L. Lawrence, Celestial Calculations, 2018, page 180

        // Arrange
        let date = Date::new(2015, 1, 1.0);
        let jd = JD::from_date(date);

        // Act
        let percent_illuminated = fraction_illuminated(jd) * 100.0;

        // Assert
        assert_approx_eq!(82.43, percent_illuminated, 0.01)
    }

    #[test]
    fn phase_description_test_1() {
        // Arrange

        // SS: Dec. 4th, 2021, 12:26PM local Denver time
        let jd = JD::new(2_459_553.3);

        // Act
        let phase_desc = phase_description(jd);

        // Assert
        assert_eq!("New Moon", phase_desc)
    }

    #[test]
    fn phase_description_test_2() {
        // Arrange

        // SS: Dec. 8th, 2021, 12:37PM local Denver time
        let jd = JD::new(2_459_557.338747);

        // Act
        let phase_desc = phase_description(jd);

        // Assert
        assert_eq!("Waxing Crescent", phase_desc)
    }

    #[test]
    fn phase_description_test_3() {
        // Arrange

        // SS: Dec. 30th, 2021, 9:30PM local Denver time
        let jd = JD::new(2_459_580.187);

        // Act
        let phase_desc = phase_description(jd);

        // Assert
        assert_eq!("Waning Crescent", phase_desc)
    }

    #[test]
    fn phase_description_test_4() {
        // J.L. Lawrence, Celestial Calculations, 2018, page 180

        // Arrange
        let date = Date::new(2015, 1, 1.0);
        let jd = JD::from_date(date);

        // Act
        let phase_desc = phase_description(jd);

        // Assert
        assert_eq!("Waxing Gibbous", phase_desc)
    }
}
