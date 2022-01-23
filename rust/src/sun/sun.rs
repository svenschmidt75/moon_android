use crate::jd;
use crate::util::{degrees::Degrees};

/// Calculate the sun's mean anomaly, eq (47.3).
/// In: Julian day in dynamical time
/// Out: Sun's mean anomaly in degrees, [0, 360)
pub fn mean_anomaly(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;

    let mean_anomaly =
        Degrees::new(357.5291092 + 35999.0502909 * t - 0.0001536 * t2 + t3 / 24_490_000.0);

    let mapped = mean_anomaly.map_to_0_to_360();
    mapped
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn sun_mean_anomaly_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12.0);

        // Act
        let mean_elongation = mean_anomaly(jd);

        // Assert
        assert_approx_eq!(97.643514, mean_elongation.0, 0.000_001)
    }
}
