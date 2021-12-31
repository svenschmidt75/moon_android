//! Utility functions

use std::ops::Sub;

const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;
const RADIANS_TO_DEGREES: f64 = 1.0 / DEGREES_TO_RADIANS;

/// Map angle in degrees to range [0, 360)
pub fn map_to_0_to_360(angle: Degrees) -> Degrees {
    let mut m = angle.0 % 360.0;
    if m < 0.0 {
        m += 360.0;
    }
    Degrees::new(m)
}

/// Map angle in radians to range [0, 2 pi)
pub fn map_to_0_to_2pi(angle: Radians) -> Radians {
    let mut m = angle.0 % (2.0 * std::f64::consts::PI);
    if m < 0.0 {
        m += 2.0 * std::f64::consts::PI;
    }
    Radians::new(m)
}

pub fn map_to_neg90_to_90(angle: Degrees) -> Degrees {
    Degrees(angle.0 % 90.0)
}

/// Convert from degrees [0, 360) to [0, 2 pi)
// pub fn to_radians(angle: f64) -> f64 {
//     angle * DEGREES_TO_RADIANS
// }

/// Convert from radians [0, 2 pi) to [0, 360)
// pub fn to_degrees(angle: f64) -> f64 {
//     angle * RADIANS_TO_DEGREES
// }

/// Convert from arcsec to degrees
// pub fn arcsec_to_degrees(v: f64) -> f64 {
//     // SS: a degree has 3600 arcsec
//     v / (60.0 * 60.0)
// }

/// Convert from degrees to hours
/// In: angle in degrees [0, 360)
// pub fn degrees_to_hours(angle: f64) -> f64 {
//     // SS: 1 hours is 24 / 360 degrees
//     const F: f64 = 24.0 / 360.0;
//     angle * F
// }

/// Convert from degrees to hours
/// In: angle in radians [0, 2 pi)
// pub fn radians_to_hours(angle: f64) -> f64 {
//     const F: f64 = 24.0 / (2.0 * std::f64::consts::PI);
//     angle * F
// }

#[derive(Debug, Clone, Copy)]
pub struct ArcSec(f64);

impl ArcSec {
    pub fn new(arcsec: f64) -> Self {
        Self(arcsec)
    }

    pub fn new_from_degrees(degrees: i16, minutes: i16, seconds: f64) -> Self {
        let arcsec = seconds + 60.0 * (minutes as f64 + 60.0 * degrees as f64);
        Self(arcsec)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Degrees(pub(crate) f64);

impl Degrees {
    pub fn new(degrees: f64) -> Self {
        Self(degrees)
    }
}

impl Sub for Degrees {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Degrees(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Radians(pub(crate) f64);

impl Radians {
    pub fn new(radians: f64) -> Self {
        Self(radians)
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: ArcSec) -> Self {
        let radians = degrees * DEGREES_TO_RADIANS;
        Self(radians)
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        let degrees = radians.0 * RADIANS_TO_DEGREES;
        Self(degrees)
    }
}

impl From<Degrees> for ArcSec {
    fn from(degrees: ArcSec) -> Self {
        let degrees = arcsec * 3600.0;
        Self(degrees)
    }
}

impl From<ArcSec> for Degrees {
    fn from(arcsec: ArcSec) -> Self {
        // SS: 1 arcsec = 1/3600 degree
        let degrees = arcsec.0 / 3600.0;
        Self(degrees)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RA(f64);

impl From<Degrees> for RA {
    fn from(angle: Degrees) -> Self {
        // SS: 1 hours is 24 / 360 degrees
        const F: f64 = 24.0 / 360.0;
        Self(angle * F)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn arcsec_to_degrees_test_1() {
        // Arrange
        let arcsec = ArcSec::new_from_degrees(133, 10, 2.154);

        // Act
        let degrees: f64 = Degrees::from(arcsec).0;

        // Assert
        assert_approx_eq!(133.167265, degrees, 0.000_1)
    }

    #[test]
    fn arcsec_to_degrees_test_2() {
        // Arrange
        let arcsec = ArcSec::new_from_degrees(23, 26, 26.29);

        // Act
        let degrees: f64 = Degrees::from(arcsec).0;

        // Assert
        assert_approx_eq!(23.440636, degrees, 0.000_001)
    }
    //
    // #[test]
    // fn degrees_to_arc_test() {
    //     // Arrange
    //     let angle = 133.167265;
    //
    //     // Act
    //     let arcsec = ArcSec::from(angle);
    //
    //     // Assert
    //     assert_eq!(133, arcsec.degrees);
    //     assert_eq!(10, arcsec.minutes);
    //     assert_approx_eq!(2.154, arcsec.seconds, 0.001);
    // }
    //
    // #[test]
    // fn map_negative_1() {
    //     // Arrange
    //     let angle = -10.0;
    //
    //     // Act
    //     let mapped = map_to_0_to_360(angle);
    //
    //     // Assert
    //     assert_eq!(360.0 + angle, mapped)
    // }
    //
    // #[test]
    // fn degrees_to_hours_test() {
    //     // Arrange
    //     let angle = 134.688470;
    //
    //     // Act
    //     let hours = degrees_to_hours(angle);
    //
    //     // Assert
    //     assert_approx_eq!(8.979, hours, 0.001)
    // }
}
