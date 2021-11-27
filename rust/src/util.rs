//! Utility functions

/// Map angle in degrees to range [0, 260)
pub fn map_to_0_to_360(angle: f64) -> f64 {
    let mut m = angle % 360.0;
    if m < 0.0 {
        m += 360.0;
    }
    m
}

const DEGREES_TO_RADIANS: f64  = std::f64::consts::PI / 180.0;

// Convert from degrees [0, 360) to [0, 2 pi)
pub fn to_radians(angle: f64) -> f64 {
    angle * DEGREES_TO_RADIANS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_negative_1() {
        // Arrange
        let angle = -10.0;

        // Act
        let mapped = map_to_0_to_360(angle);

        // Assert
        assert_eq!(360.0 + angle, mapped)
    }
}
