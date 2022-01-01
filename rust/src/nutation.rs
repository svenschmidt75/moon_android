//! Solar system related calculations.
use crate::jd;
use crate::util::{ArcSec, Degrees, Radians};

const NUTATION_PERTURBATION_TERMS: [(i8, i8, i8, i8, i8, i64, f64, i64, f64); 63] = [
    (0, 0, 0, 0, 1, -171996, -174.2, 92025, 8.9),
    (-2, 0, 0, 2, 2, -13187, -1.6, 5736, -3.1),
    (0, 0, 0, 2, 2, -2274, -0.2, 977, -0.5),
    (0, 0, 0, 0, 2, 2062, 0.2, -895, 0.5),
    (0, 1, 0, 0, 0, 1426, -3.4, 54, -0.1),
    (0, 0, 1, 0, 0, 712, 0.1, -7, 0.0),
    (-2, 1, 0, 2, 2, -517, 1.2, 224, -0.6),
    (0, 0, 0, 2, 1, -386, -0.4, 200, 0.0),
    (0, 0, 1, 2, 2, -301, 0.0, 129, -0.1),
    (-2, -1, 0, 2, 2, 217, -0.5, -95, 0.3),
    (-2, 0, 1, 0, 0, -158, 0.0, 0, 0.0),
    (-2, 0, 0, 2, 1, 129, 0.1, -70, 0.0),
    (0, 0, -1, 2, 2, 123, 0.0, -53, 0.0),
    (2, 0, 0, 0, 0, 63, 0.0, 0, 0.0),
    (0, 0, 1, 0, 1, 63, 0.1, -33, 0.0),
    (2, 0, -1, 2, 2, -59, 0.0, 26, 0.0),
    (0, 0, -1, 0, 1, -58, -0.1, 32, 0.0),
    (0, 0, 1, 2, 1, -51, 0.0, 27, 0.0),
    (-2, 0, 2, 0, 0, 48, 0.0, 0, 0.0),
    (0, 0, -2, 2, 1, 46, 0.0, -24, 0.0),
    (2, 0, 0, 2, 2, -38, 0.0, 16, 0.0),
    (0, 0, 2, 2, 2, -31, 0.0, 13, 0.0),
    (0, 0, 2, 0, 0, 29, 0.0, 0, 0.0),
    (-2, 0, 1, 2, 2, 29, 0.0, -12, 0.0),
    (0, 0, 0, 2, 0, 26, 0.0, 0, 0.0),
    (-2, 0, 0, 2, 0, -22, 0.0, 0, 0.0),
    (0, 0, -1, 2, 1, 21, 0.0, -10, 0.0),
    (0, 2, 0, 0, 0, 17, -0.1, 0, 0.0),
    (2, 0, -1, 0, 1, 16, 0.0, -8, 0.0),
    (-2, 2, 0, 2, 2, -16, 0.1, 7, 0.0),
    (0, 1, 0, 0, 1, -15, 0.0, 9, 0.0),
    (-2, 0, 1, 0, 1, -13, 0.0, 7, 0.0),
    (0, -1, 0, 0, 1, -12, 0.0, 6, 0.0),
    (0, 0, 2, -2, 0, 11, 0.0, 0, 0.0),
    (2, 0, -1, 2, 1, -10, 0.0, 5, 0.0),
    (2, 0, 1, 2, 2, -8, 0.0, 3, 0.0),
    (0, 1, 0, 2, 2, 7, 0.0, -3, 0.0),
    (-2, 1, 1, 0, 0, -7, 0.0, 0, 0.0),
    (0, -1, 0, 2, 2, -7, 0.0, 3, 0.0),
    (2, 0, 0, 2, 1, -7, 0.0, 3, 0.0),
    (2, 0, 1, 0, 0, 6, 0.0, 0, 0.0),
    (-2, 0, 2, 2, 2, 6, 0.0, -3, 0.0),
    (-2, 0, 1, 2, 1, 6, 0.0, -3, 0.0),
    (2, 0, -2, 0, 1, -6, 0.0, 3, 0.0),
    (2, 0, 0, 0, 1, -6, 0.0, 3, 0.0),
    (0, -1, 1, 0, 0, 5, 0.0, 0, 0.0),
    (-2, -1, 0, 2, 1, -5, 0.0, 3, 0.0),
    (-2, 0, 0, 0, 1, -5, 0.0, 3, 0.0),
    (0, 0, 2, 2, 1, -5, 0.0, 3, 0.0),
    (2, 0, 2, 0, 1, 4, 0.0, 0, 0.0),
    (2, 1, 0, 2, 1, 4, 0.0, 0, 0.0),
    (0, 0, 1, -2, 0, 4, 0.0, 0, 0.0),
    (-1, 0, 1, 0, 0, -4, 0.0, 0, 0.0),
    (-2, 1, 0, 0, 0, -4, 0.0, 0, 0.0),
    (1, 0, 0, 0, 0, -4, 0.0, 0, 0.0),
    (0, 0, 1, 2, 0, 3, 0.0, 0, 0.0),
    (0, 0, -2, 2, 2, -3, 0.0, 0, 0.0),
    (-1, -1, 1, 0, 0, -3, 0.0, 0, 0.0),
    (0, 1, 1, 0, 0, -3, 0.0, 0, 0.0),
    (0, -1, 1, 2, 2, -3, 0.0, 0, 0.0),
    (2, -1, -1, 2, 2, -3, 0.0, 0, 0.0),
    (0, 0, 3, 2, 2, -3, 0.0, 0, 0.0),
    (2, -1, 0, 2, 2, -3, 0.0, 0, 0.0),
];

/// Nutation of the Earth, Meeus chapter 22
/// In: Julian day in dynamical time
/// Out: correction term, in arcsec
pub fn nutation_in_longitude(jd: f64) -> ArcSec {
    let t = jd::centuries_from_epoch_j2000(jd);
    let t2 = t * t;
    let t3 = t * t2;

    let d = Degrees::new(297.85036 + (445267.111480 * t) - (0.0019142 * t2) + (t3 / 189_474.0))
        .map_to_0_to_360();
    let m = Degrees::new(357.52772 + (35_999.050340 * t) - (0.0001603 * t2) - (t3 / 300_000.0))
        .map_to_0_to_360();

    let m_prime =
        Degrees::new(134.96298 + (477_198.867398 * t) + (0.0086972 * t2) + (t3 / 56_250.0))
            .map_to_0_to_360();

    let f = Degrees::new(93.27191 + (483_202.017538 * t) - (0.0036825 * t2) + (t3 / 327_270.0))
        .map_to_0_to_360();

    let omega = Degrees::new(125.04452 - (1934.136261 * t) + (0.0020708 * t2) + (t3 / 450_000.0))
        .map_to_0_to_360();

    let delta_psi = NUTATION_PERTURBATION_TERMS.iter().fold(0.0, |accum, &c| {
        let sin_arg = c.0 as f64 * d.0
            + c.1 as f64 * m.0
            + c.2 as f64 * m_prime.0
            + c.3 as f64 * f.0
            + c.4 as f64 * omega.0;
        let sin_arg = Radians::from(Degrees::new(sin_arg));
        let value = (c.5 as f64 + c.6 * t) * sin_arg.0.sin() * 0.0001;
        accum + value
    });

    ArcSec::new(delta_psi)
}
/// Nutation of the obliquity of the eclipse, Meeus chapter 22
/// In: Julian day in dynamical time
/// Out: correction term in arcsec
pub fn nutation_in_obliquity(jd: f64) -> ArcSec {
    let t = jd::centuries_from_epoch_j2000(jd);
    let t2 = t * t;
    let t3 = t * t2;

    let d = Degrees::new(297.85036 + (445267.111480 * t) - (0.0019142 * t2) + (t3 / 189_474.0))
        .map_to_0_to_360();
    let m = Degrees::new(357.52772 + (35_999.050340 * t) - (0.0001603 * t2) - (t3 / 300_000.0))
        .map_to_0_to_360();
    let m_prime =
        Degrees::new(134.96298 + (477_198.867398 * t) + (0.0086972 * t2) + (t3 / 56_250.0))
            .map_to_0_to_360();
    let f = Degrees::new(93.27191 + (483_202.017538 * t) - (0.0036825 * t2) + (t3 / 327_270.0))
        .map_to_0_to_360();
    let omega = Degrees::new(125.04452 - (1934.136261 * t) + (0.0020708 * t2) + (t3 / 450_000.0))
        .map_to_0_to_360();

    let delta_epsilon = NUTATION_PERTURBATION_TERMS.iter().fold(0.0, |accum, &c| {
        let cos_arg = c.0 as f64 * d.0
            + c.1 as f64 * m.0
            + c.2 as f64 * m_prime.0
            + c.3 as f64 * f.0
            + c.4 as f64 * omega.0;
        let cos_arg = Radians::from(Degrees::new(cos_arg));
        let value = (c.7 as f64 + c.8 * t) * cos_arg.0.cos() * 0.0001;
        accum + value
    });

    ArcSec::new(delta_epsilon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn nutation_longitude_test_1() {
        // Arrange
        let jd = jd::from_date(1987, 4, 10, 0.0);

        // Act
        let delta_psi = nutation_in_longitude(jd);

        // Assert
        assert_approx_eq!(-3.788, delta_psi.0, 0.001)
    }

    #[test]
    fn nutation_longitude_test_2() {
        // Arrange

        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let delta_psi = Degrees::from(nutation_in_longitude(jd));

        // Assert
        assert_approx_eq!(0.00461, delta_psi.0, 0.001)
    }

    #[test]
    fn nutation_obliquity_test_1() {
        // Arrange
        let jd = jd::from_date(1987, 4, 10, 0.0);

        // Act
        let delta_epsilon = nutation_in_obliquity(jd);

        // Assert
        assert_approx_eq!(9.443, delta_epsilon.0, 0.001)
    }
}
