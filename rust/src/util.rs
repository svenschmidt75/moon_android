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

/// Map degrees to degrees:min:sec
/// In: value in [0, 360)
pub fn degrees_to_arc(value: f64) -> (f64, f64, f64) {
    let degrees = value.trunc();
    let remainder = value - degrees;
    let minutes = (remainder * 60.0).trunc();

    let remainder = remainder * 60.0 - minutes;
    let seconds = (remainder * 60.0).trunc();
    (degrees, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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
    fn degrees_to_arc_test() {
        // Arrange
        let angle = 133.167265;

        // Act
        let (degrees, minutes, seconds) = degrees_to_arc(angle);

        // Assert
        assert_eq!(133.0, degrees);
        assert_eq!(10.0, minutes);
        assert_approx_eq!(2.0, seconds, 1.0);
    }
}
