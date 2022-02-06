//! Calculations related to the ecliptic

use crate::date::jd::JD;
use crate::nutation::nutation_in_obliquity;
use crate::util::{arcsec::ArcSec, degrees::Degrees};

/// Mean obliquity of the eclipse, Meeus chapter 22
/// In: Julian day in dynamical time
/// Out: Mean obliquity of the eclipse in degrees [0, 360)
pub fn mean_obliquity(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();
    let u = t / 100.0;

    let arcsec = ArcSec::from_dms(23, 26, 21.448);
    let eps_base: f64 = Degrees::from(arcsec).0;

    let eps_0 = eps_base
        + u / (60.0 * 60.0)
            * (-4680.93
                + u * (-1.55
                    + u * (1999.25
                        + u * (-51.38
                            + u * (-249.67
                                + u * (-39.05
                                    + u * (7.12 + u * (27.87 + u * (5.79 + u * (2.45))))))))));

    Degrees::new(eps_0)
}
/// True obliquity of the eclipse, taking into account the
/// nutation effect. Meeus chapter 22
/// In: Julian day in dynamical time
/// Out: True obliquity of the eclipse in degrees [0, 360)
pub fn true_obliquity(jd: JD) -> Degrees {
    let nutation_effect = Degrees::from(nutation_in_obliquity(jd));
    mean_obliquity(jd) + nutation_effect
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::date::Date;
    use crate::date::jd::JD;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn mean_obliquity_test() {
        // Arrange
        let jd = JD::from_date(Date::new(1987, 4, 10.0));

        // Act
        let eps = mean_obliquity(jd);

        // Assert
        assert_approx_eq!(23.44094629, eps.0, 0.000_000_001)
    }

    #[test]
    fn true_obliquity_test_1() {
        // Arrange
        let jd = JD::from_date(Date::new(1987, 4, 10.0));

        // Act
        let eps = true_obliquity(jd);

        // Assert
        assert_approx_eq!(23.44356921, eps.0, 0.000_000_01)
    }

    #[test]
    fn true_obliquity_test_2() {
        // Arrange
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let eps = true_obliquity(jd);

        // Assert
        assert_approx_eq!(23.44063489239479, eps.0, 0.000_001)
    }
}
