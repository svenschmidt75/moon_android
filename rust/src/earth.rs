use crate::{jd};

/// Calculate Earth's eccentricity, eq (47.6).
/// In: Julian day in dynamical time
pub fn eccentricity(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);
    let t2 = t * t;

    1.0 - 0.002516 * t - 0.0000074 * t2
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn eccentricity_test() {
        // Arrange

        // SS: April 12th, 1992, 0h TD
        let jd = 2_448_724.5;

        // Act
        let e = eccentricity(jd);

        // Assert
        assert_approx_eq!(1.000194, e, 0.000001)
    }
}
