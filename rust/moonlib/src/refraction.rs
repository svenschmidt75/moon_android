//! Refraction-related calculation

use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

/// Given the true altitude of an object and atmospheric conditions,
/// calculate the refraction, i.e. the correction in altitude to get the
/// apparent altitude.
/// Meeus, chapter 16, page 106
/// In:
/// altitude, in degrees [0, 90)
/// pressure: atmospheric pressure, in millibars
/// temperature, in celsius
/// Out:
/// Correction for altitude, in degrees [0, 360)
pub(crate) fn refraction_from_apparent_altitude(
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
