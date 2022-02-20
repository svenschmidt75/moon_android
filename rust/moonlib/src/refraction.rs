//! Refraction-related calculation

use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

/// Given the true altitude of an object and atmospheric conditions,
/// calculate the refraction, i.e. the correction in altitude to get the
/// apparent altitude. To do so, add the value returned to the true
/// altitude of an object.
/// Meeus, chapter 16, page 106
/// In:
/// altitude, in degrees [0, 90)
/// pressure: atmospheric pressure, in millibars
/// temperature, in celsius
/// Out:
/// Correction for altitude, in degrees [0, 360)
pub(crate) fn refraction_for_true_altitude(
    altitude: Degrees,
    pressure: f64,
    temperature: f64,
) -> Degrees {
    // SS: not sure where this constant comes from, taken from PJ Naughter's Astronomical Algorithms
    let h = if altitude.0 <= -1.9006387000003735 {
        Degrees::new(-1.9006387000003735)
    } else {
        altitude
    };

    // SS: equ (16.4)
    let r = 1.02
        / (Radians::from(Degrees::new(h.0 + 10.3 / (h.0 + 5.11))))
            .0
            .atan()
        + 0.0019279;
    let d = pressure / 1010.0 * 283.0 / (273.0 + temperature);
    let refraction = r * d;

    // SS: refraction is in minutes of arc
    let refraction_degrees = refraction / 60.0;

    Degrees::new(refraction_degrees)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn refraction_for_true_altitude_test_1() {
        // Arrange
        let height = Degrees::new(0.0);

        // Act
        let refraction = refraction_for_true_altitude(height, 1013.0, 10.0);

        // Assert
        assert_approx_eq!(0.4845, refraction.0, 0.001);
    }

    #[test]
    fn refraction_for_true_altitude_test_2() {
        // Astronomie mit dem Personal Computer, Montenbruck, Pfleger, 2004
        // On page 45, they mention the effect of refraction at the horizon
        // is about 34'.

        // Arrange
        let height = Degrees::new(0.0);

        // Act
        let (d, m, s) = refraction_for_true_altitude(height, 1013.0, 10.0).to_dms();

        // Assert
        assert_eq!(0, d);
        assert_eq!(29, m);
        assert_approx_eq!(5.636, s, 0.001);
    }
}
