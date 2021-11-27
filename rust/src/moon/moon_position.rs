//! Calculate the moon's position for given Julian day.
//! see J. Meeus, Astronomical Algorithms, chapter 47
use super::super::jd;
use super::super::util;
use assert_approx_eq::assert_approx_eq;

/// Calculate the moon's mean longitude, eq (47.1).
/// In: Julian day in dynamical time
/// Out: Moon's mean longitude in degrees, [0, 360)
fn mean_longitude(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_longitude =
        218.3164477 + 481_267.88123421 * t - 0.0015786 * t2 + t3 / 538_841.0 - t4 / 65_194_000.0;

    let mapped = util::map_to_0_to_360(mean_longitude);
    mapped
}

/// Calculate the moon's mean elongation, eq (47.2).
/// In: Julian day in dynamical time
/// Out: Moon's mean elongation in degrees, [0, 360)
fn mean_elongation(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_elongation =
        297.8501921 + 445_267.1114034 * t - 0.0018819 * t2 + t3 / 545_868.0 - t4 / 113_065_000.0;

    let mapped = util::map_to_0_to_360(mean_elongation);
    mapped
}

/// Calculate the sun's mean anomaly, eq (47.3).
/// In: Julian day in dynamical time
/// Out: Sun's mean anomaly in degrees, [0, 360)
fn sun_mean_anomaly(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;

    let mean_anomaly = 357.5291092 + 35999.0502909 * t - 0.0001536 * t2 + t3 / 24_490_000.0;

    let mapped = util::map_to_0_to_360(mean_anomaly);
    mapped
}

/// Calculate Earth's eccentricity, eq (47.6).
/// In: Julian day in dynamical time
fn earth_eccentricity(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);
    let t2 = t * t;

    1.0 - 0.002516 * t - 0.0000074 * t2
}

/// Calculate the moon's mean anomaly, eq (47.4).
/// In: Julian day in dynamical time
/// Out: Moon's mean anomaly in degrees, [0, 360)
fn mean_anomaly(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_anomaly =
        134.9633964 + 477198.8675055 * t + 0.0087414 * t2 + t3 / 69_699.0 - t4 / 14_712_000.0;

    let mapped = util::map_to_0_to_360(mean_anomaly);
    mapped
}

/// Calculate the moon's argument of latitude, eq (47.5).
/// In: Julian day in dynamical time
/// Out: Moon's argument of latitude in degrees, [0, 360)
fn argument_of_latitude(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let argument_of_latitude =
        93.2720950 + 483202.0175233 * t - 0.0036539 * t2 - t3 / 3_526_000.0 + t4 / 863_310_000.0;

    let mapped = util::map_to_0_to_360(argument_of_latitude);
    mapped
}

// SS: perturbation terms for longitude and radius
const SIGMA_L_AND_R_COEFFICIENTS: [(i8, i8, i8, i8, i64, i64); 1] =
    [(0, 0, 1, 0, 6_288_744, -20_905_355)];

// SS: perturbation terms for latitude
const SIGMA_B_COEFFICIENTS: [(i8, i8, i8, i8, i64); 1] = [(0, 0, 0, 1, 5_128_122)];

/// Calculate the moon's longitude (lambda), page 342
/// In: Julian day in dynamical time
/// Out: Moon's longitude in degrees, [0, 360)
fn longitude(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let a1 = util::to_radians(util::map_to_0_to_360(119.75 + 131.849 * t));
    let a2 = util::to_radians(util::map_to_0_to_360(53.09 + 479264.290 * t));

    let l_prime = mean_longitude(jd);
    let d = mean_elongation(jd);
    let m = sun_mean_anomaly(jd);
    let m_prime = mean_anomaly(jd);
    let f = argument_of_latitude(jd);
    let e = earth_eccentricity(jd);

    // SS: perturbation term for moon's longitude
    let mut sigma_l = SIGMA_L_AND_R_COEFFICIENTS.iter().fold(0.0, |accum, &c| {
        let sin_arg = c.0 as f64 * d + c.1 as f64 * m + c.2 as f64 * m_prime + c.3 as f64 * f;
        let mut coeff = c.4 as f64;

        if c.1 != 0 {
            coeff *= e;
        }

        if c.1 == -2 || c.1 == 2 {
            coeff *= e;
        }

        accum + coeff * sin_arg.sin()
    });

    sigma_l += 3958.0 * a1.sin();
    sigma_l += 1962.0 * (l_prime - f).sin();
    sigma_l += 318.0 * a2.sin();

    l_prime + sigma_l / 1_000_000.0
}

/// Calculate the moon's latitude (beta), page 342
/// In: Julian day in dynamical time
/// Out: Moon's latitude in degrees, [0, 360)
fn latitude(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let a1 = util::to_radians(util::map_to_0_to_360(119.75 + 131.849 * t));
    let a2 = util::to_radians(util::map_to_0_to_360(53.09 + 479264.290 * t));
    let a3 = util::to_radians(util::map_to_0_to_360(313.45 + 481266.484 * t));

    let l_prime = mean_longitude(jd);
    let d = mean_elongation(jd);
    let m = sun_mean_anomaly(jd);
    let m_prime = mean_anomaly(jd);
    let f = argument_of_latitude(jd);
    let e = earth_eccentricity(jd);

    // SS: perturbation term for moon's latitude
    let mut sigma_b = SIGMA_B_COEFFICIENTS.iter().fold(0.0, |accum, &c| {
        let cos_arg = c.0 as f64 * d + c.1 as f64 * m + c.2 as f64 * m_prime + c.3 as f64 * f;
        let mut coeff = c.4 as f64;

        if c.1 != 0 {
            coeff *= e;
        }

        if c.1 == -2 || c.1 == 2 {
            coeff *= e;
        }

        accum + coeff * cos_arg.cos()
    });

    sigma_b -= 2235.0 * l_prime.sin();
    sigma_b += 382.0 * a3.sin();
    sigma_b += 175.0 * (a1 - f).sin();
    sigma_b += 175.0 * (a1 + f).sin();
    sigma_b += 127.0 * (l_prime - m_prime).sin();
    sigma_b -= 115.0 * (l_prime + m_prime).sin();

    sigma_b / 1_000_000.0
}

/// Calculate the moon's distance (delta) from earth, page 342
/// In: Julian day in dynamical time
/// Out: Moon's distance from Earth, in kilometers
fn distance_from_earth(jd: f64) -> f64 {
    let t = jd::from_epoch_j2000(jd);

    let l_prime = mean_longitude(jd);
    let d = mean_elongation(jd);
    let m = sun_mean_anomaly(jd);
    let m_prime = mean_anomaly(jd);
    let f = argument_of_latitude(jd);
    let e = earth_eccentricity(jd);

    // SS: perturbation term for moon's longitude
    let mut sigma_r = SIGMA_L_AND_R_COEFFICIENTS.iter().fold(0.0, |accum, &c| {
        let cos_arg = c.0 as f64 * d + c.1 as f64 * m + c.2 as f64 * m_prime + c.3 as f64 * f;
        let mut coeff = c.5 as f64;

        if c.1 != 0 {
            coeff *= e;
        }

        if c.1 == -2 || c.1 == 2 {
            coeff *= e;
        }

        accum + coeff * cos_arg.cos()
    });

    // SS: 385,000.56 is the mean distance Earth-Moon,
    // now add the perturbation term
    385_000.56 + sigma_r / 1000.0
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

    #[test]
    fn mean_elongation_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let mean_elongation = mean_elongation(jd);

        // Assert
        assert_approx_eq!(113.842304, mean_elongation, 0.000_001)
    }

    #[test]
    fn sun_mean_anomaly_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let mean_elongation = sun_mean_anomaly(jd);

        // Assert
        assert_approx_eq!(97.643514, mean_elongation, 0.000_001)
    }

    #[test]
    fn mean_anomaly_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let mean_elongation = mean_anomaly(jd);

        // Assert
        assert_approx_eq!(5.150833, mean_elongation, 0.000_001)
    }

    #[test]
    fn argument_of_latitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let argument_of_latitude = argument_of_latitude(jd);

        // Assert
        assert_approx_eq!(219.889721, argument_of_latitude, 0.000_001)
    }
}
