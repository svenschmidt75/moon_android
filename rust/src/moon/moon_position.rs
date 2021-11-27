//! Calculate the moon's position for given Julian day.
/// see J. Meeus, Astronomical Algorithms, chapter 47
use assert_approx_eq::assert_approx_eq;
use super::super::jd;
use super::super::util;

/// Calculate the moon's mean longitude, eq (47.1).
/// In: Julian day in dynamical time
/// Out: Moon's mean longitude in degrees, [0, 360)
fn mean_longitude(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_longitude = 218.3164477 + 481_267.88123421 * t - 0.0015786 * t2 + t3 / 538_841.0
        - t4 / 65_194_000.0;

    let mapped = util::map_to_0_to_360(mean_longitude);
    mapped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_longitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let mean_longitude = mean_longitude(jd);

        // Assert
        assert_approx_eq!(134.290182, mean_longitude, 0.000_001)
    }
}
