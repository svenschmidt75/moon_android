use crate::ecliptic::true_obliquity;
use crate::nutation::nutation_in_longitude;
use crate::util::{Degrees, Radians};

/// Calculate the mean siderial time at Greenwich
/// In: Julian Day
/// Out: Mean siderial time in degrees [0, 360)
pub fn mean_siderial_time(jd: f64) -> Degrees {
    let delta_jd = jd - 2_451_545.0;
    let t = delta_jd / 36525.0;
    let t2 = t * t;
    let t3 = t * t2;
    let mean_siderial_time =
        280.46061836 + 360.98564736629 * delta_jd + 0.000387933 * t2 - t3 / 38_710_000.0;
    Degrees(mean_siderial_time).map_to_0_to_360()
}

/// Calculate
pub fn apparent_siderial_time(jd: f64) -> Degrees {
    let mean_siderial_time = mean_siderial_time(jd);
    let eps = true_obliquity(jd);
    let delta_psi = nutation_in_longitude(jd);

    let siderial_time = mean_siderial_time + Degrees::from(delta_psi) * Radians::from(eps).0.cos();
    siderial_time
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn mean_siderial_time_test_1() {
        // Arrange

        // SS: Jan 16th, 2022, 2:26:18pm UTC
        let jd = 2_459_596.101598;

        // Act
        let theta0 = mean_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 22);
        assert_eq!(m, 10);
        assert_approx_eq!(19.92073, s, 0.00001)
    }

    #[test]
    fn mean_siderial_time_test_2() {
        // Meeus, example 12.b, page 89

        // Arrange

        // SS: Apr. 10th 1987, 19h:21m:00s UT
        let jd = 2_446_896.30625;

        // Act
        let theta0 = mean_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_approx_eq!(128.7378734, theta0.0, 0.00001);

        assert_eq!(h, 8);
        assert_eq!(m, 34);
        assert_approx_eq!(57.0896, s, 0.0001)
    }

    #[test]
    fn apparent_siderial_time_test_1() {
        // Arrange

        // SS: Jan 16th, 2022, 2:26:18pm UTC
        let jd = 2_459_596.101598;

        // Act
        let theta0 = apparent_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 22);
        assert_eq!(m, 10);
        assert_approx_eq!(19.10356, s, 0.00001)
    }

    #[test]
    fn apparent_siderial_time_test_2() {
        // Meeus, example 12.a, page 88

        // Arrange

        // SS: Apr. 10th 1987, 0 UT
        let jd = 2_446_895.5;

        // Act
        let theta0 = apparent_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 13);
        assert_eq!(m, 10);
        assert_approx_eq!(46.1351, s, 0.000_1)
    }
}
