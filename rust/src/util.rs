//! Utility functions

/// Map angle in degrees to range [0, 260)
pub fn map_to_0_to_360(angle: f64) -> f64 {
    let mut m = angle % 360.0;
    if m < 0.0 {
        m += 360.0;
    }
    m
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
