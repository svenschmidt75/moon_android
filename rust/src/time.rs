//! Time-related function.
//!
//! Siderial Day: Imagine a reference longitudinal half-circle at noon where the Sun is in transit, i.e.
//! the sun is at the zenith crossing the observer's meridian. Now the Earth keeps rotating around its
//! axis, but it also moves in its orbit around the sun. After Earth rotates by 360 degrees, the sun will
//! not be at the zenith again. This 360 degree "day" is called a siderial day, i.e. the stars are at the
//! same position as before.
//
// The Earth has to rotate more than 360 degrees for the sun to be at the zenith again. This is called a
// solar day.
//
// see https://www.youtube.com/watch?v=1wGFJd3j3ds
//
// The length of a solar day varies throughout the year, as the Earth moves around an eclipse, not a
// perfect circle. Siderial days are always the same length, as they are defined by Earth rotating
// once around its axis.
use crate::ecliptic::true_obliquity;
use crate::nutation::nutation_in_longitude;
use crate::util::{Degrees, Radians};

/// Calculate the mean siderial time at Greenwich
/// Meeus, page 87, chapter 12
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

/// Calculate the apparent siderial time at Greenwich, which
/// takes Earth's nutation effects into account.
/// Meeus, page 87, chapter 12
/// In: Julian Day
/// Out: Mean siderial time in degrees [0, 360)
pub fn apparent_siderial_time(jd: f64) -> Degrees {
    let mean_siderial_time = mean_siderial_time(jd);
    let eps = true_obliquity(jd);
    let delta_psi = nutation_in_longitude(jd);

    let siderial_time = mean_siderial_time + Degrees::from(delta_psi) * Radians::from(eps).0.cos();
    siderial_time
}

/// Local siderial time
/// In:
/// siderial_time: Siderial time at Greenwich, either mean or apparent, in degrees [0, 360)
/// lambda_observer: Observer's longitude, in degrees [-180, 180)
/// (positive west, negative east of Greenwich)
/// Out:
/// Local siderial time
fn local_siderial_time(siderial_time: Degrees, lambda_observer: Degrees) -> Degrees {
    Degrees::new(siderial_time.0 - lambda_observer.0).map_to_0_to_360()
}

/// Calculate the local hour angle, which measures how far an object is from the observer's meridian,
/// measured westwards from south.
/// Said differently, an hour angle of 7h:21m means that this object passed the observer's meridian
/// 7h:21 minutes ago.
/// In:
/// siderial_time: Local siderial time (i.e. observer's siderial time), either mean or apparent, in degrees [0, 360)
/// right ascension: Right ascension of the object whose hour angle we calculate, in degrees [0, 360)
/// Out:
/// Hour angle
fn hour_angle(siderial_time: Degrees, right_ascension: Degrees) -> Degrees {
    Degrees::new(siderial_time.0 - right_ascension.0).map_to_0_to_360()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn local_siderial_time_test() {
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
