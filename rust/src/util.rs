//! Utility functions

/// Map angle in degrees to range [0, 260)
pub fn map_to_0_to_360(angle: f64) -> f64 {
    let mut m = angle % 360.0;
    if m < 0.0 {
        m += 360.0;
    }
    m
}

const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;
const RADIANS_TO_DEGREES: f64 = 1.0 / DEGREES_TO_RADIANS;

/// Convert from degrees [0, 360) to [0, 2 pi)
pub fn to_radians(angle: f64) -> f64 {
    angle * DEGREES_TO_RADIANS
}

/// Convert from radians [0, 2 pi) to [0, 360)
pub fn to_degrees(angle: f64) -> f64 {
    angle * RADIANS_TO_DEGREES
}

/// Convert from arcsec to degrees
pub fn arcsec_to_degrees(v: f64) -> f64 {
    // SS: a degree has 3600 arcsec
    v / (60.0 * 60.0)
}

/// Convert from degrees to hours
/// In: angle in degrees [0, 360)
pub fn degrees_to_hours(angle: f64) -> f64 {
    // SS: 1 hours is 24 / 360 degrees
    const F: f64 = 24.0 / 360.0;
    angle * F
}

/// Convert from degrees to hours
/// In: angle in radians [0, 2 pi)
pub fn radians_to_hours(angle: f64) -> f64 {
    const F: f64 = 24.0 / (2.0 * std::f64::consts::PI);
    angle * F
}

#[derive(Debug, Clone, Copy)]
pub struct ArcSec {
    pub(crate) degrees: i16,
    pub(crate) minutes: i8,
    pub(crate) seconds: f64,
}

impl From<f64> for ArcSec {
    fn from(angle: f64) -> Self {
        let degrees = angle.trunc() as i16;

        let remainder = angle - degrees as f64;
        let minutes = (remainder * 60.0).trunc() as i8;

        let seconds = (remainder * 60.0 - minutes as f64) * 60.0;

        Self {
            degrees,
            minutes,
            seconds,
        }
    }
}

impl From<ArcSec> for f64 {
    fn from(arcsec: ArcSec) -> Self {
        arcsec.degrees as f64 + arcsec.minutes as f64 / 60.0 + arcsec.seconds as f64 / (60.0 * 60.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RA {
    pub(crate) hours: i16,
    pub(crate) minutes: i8,
    pub(crate) seconds: f64,
}

impl From<f64> for RA {
    fn from(angle: f64) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn arcsec_to_degrees_test_1() {
        // Arrange
        let arcsec = ArcSec {
            degrees: 133,
            minutes: 10,
            seconds: 2.154,
        };

        // Act
        let degrees: f64 = f64::from(arcsec);

        // Assert
        assert_approx_eq!(133.167265, degrees, 0.000_1)
    }

    #[test]
    fn arcsec_to_degrees_test_2() {
        // Arrange
        let arcsec = ArcSec {
            degrees: 23,
            minutes: 26,
            seconds: 26.29,
        };

        // Act
        let degrees: f64 = f64::from(arcsec);

        // Assert
        assert_approx_eq!(23.440636, degrees, 0.000_001)
    }

    #[test]
    fn degrees_to_arc_test() {
        // Arrange
        let angle = 133.167265;

        // Act
        let arcsec = ArcSec::from(angle);

        // Assert
        assert_eq!(133, arcsec.degrees);
        assert_eq!(10, arcsec.minutes);
        assert_approx_eq!(2.154, arcsec.seconds, 0.001);
    }

    #[test]
    fn map_negative_1() {
        // Arrange
        let angle = -10.0;

        // Act
        let mapped = map_to_0_to_360(angle);

        // Assert
        assert_eq!(360.0 + angle, mapped)
    }

    #[test]
    fn degrees_to_hours_test() {
        // Arrange
        let angle = 134.688470;

        // Act
        let hours = degrees_to_hours(angle);

        // Assert
        assert_approx_eq!(8.979, hours, 0.001)
    }
}
