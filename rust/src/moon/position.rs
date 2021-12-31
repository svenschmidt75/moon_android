//! Calculate the moon's position for given Julian day.
//! see J. Meeus, Astronomical Algorithms, chapter 47
use crate::util::{Degrees, Radians};
use crate::{earth, jd, nutation, sun::sun, util};

// SS: perturbation terms for longitude and radius
const SIGMA_L_AND_R_COEFFICIENTS: [(i8, i8, i8, i8, i64, i64); 60] = [
    (0, 0, 1, 0, 6288774, -20905355),
    (2, 0, -1, 0, 1274027, -3699111),
    (2, 0, 0, 0, 658314, -2955968),
    (0, 0, 2, 0, 213618, -569925),
    (0, 1, 0, 0, -185116, 48888),
    (0, 0, 0, 2, -114332, -3149),
    (2, 0, -2, 0, 58793, 246158),
    (2, -1, -1, 0, 57066, -152138),
    (2, 0, 1, 0, 53322, -170733),
    (2, -1, 0, 0, 45758, -204586),
    (0, 1, -1, 0, -40923, -129620),
    (1, 0, 0, 0, -34720, 108743),
    (0, 1, 1, 0, -30383, 104755),
    (2, 0, 0, -2, 15327, 10321),
    (0, 0, 1, 2, -12528, 0),
    (0, 0, 1, -2, 10980, 79661),
    (4, 0, -1, 0, 10675, -34782),
    (0, 0, 3, 0, 10034, -23210),
    (4, 0, -2, 0, 8548, -21636),
    (2, 1, -1, 0, -7888, 24208),
    (2, 1, 0, 0, -6766, 30824),
    (1, 0, -1, 0, -5163, -8379),
    (1, 1, 0, 0, 4987, -16675),
    (2, -1, 1, 0, 4036, -12831),
    (2, 0, 2, 0, 3994, -10445),
    (4, 0, 0, 0, 3861, -11650),
    (2, 0, -3, 0, 3665, 14403),
    (0, 1, -2, 0, -2689, -7003),
    (2, 0, -1, 2, -2602, 0),
    (2, -1, -2, 0, 2390, 10056),
    (1, 0, 1, 0, -2348, 6322),
    (2, -2, 0, 0, 2236, -9884),
    (0, 1, 2, 0, -2120, 5751),
    (0, 2, 0, 0, -2069, 0),
    (2, -2, -1, 0, 2048, -4950),
    (2, 0, 1, -2, -1773, 4130),
    (2, 0, 0, 2, -1595, 0),
    (4, -1, -1, 0, 1215, -3958),
    (0, 0, 2, 2, -1110, 0),
    (3, 0, -1, 0, -892, 3258),
    (2, 1, 1, 0, -810, 2616),
    (4, -1, -2, 0, 759, -1897),
    (0, 2, -1, 0, -713, -2117),
    (2, 2, -1, 0, -700, 2354),
    (2, 1, -2, 0, 691, 0),
    (2, -1, 0, -2, 596, 0),
    (4, 0, 1, 0, 549, -1423),
    (0, 0, 4, 0, 537, -1117),
    (4, -1, 0, 0, 520, -1571),
    (1, 0, -2, 0, -487, -1739),
    (2, 1, 0, -2, -399, 0),
    (0, 0, 2, -2, -381, -4421),
    (1, 1, 1, 0, 351, 0),
    (3, 0, -2, 0, -340, 0),
    (4, 0, -3, 0, 330, 0),
    (2, -1, 2, 0, 327, 0),
    (0, 2, 1, 0, -323, 1165),
    (1, 1, -1, 0, 299, 0),
    (2, 0, 3, 0, 294, 0),
    (2, 0, -1, -2, 0, 8752),
];

// SS: perturbation terms for latitude
const SIGMA_B_COEFFICIENTS: [(i8, i8, i8, i8, i64); 60] = [
    (0, 0, 0, 1, 5128122),
    (0, 0, 1, 1, 280602),
    (0, 0, 1, -1, 277693),
    (2, 0, 0, -1, 173237),
    (2, 0, -1, 1, 55413),
    (2, 0, -1, -1, 46271),
    (2, 0, 0, 1, 32573),
    (0, 0, 2, 1, 17198),
    (2, 0, 1, -1, 9266),
    (0, 0, 2, -1, 8822),
    (2, -1, 0, -1, 8216),
    (2, 0, -2, -1, 4324),
    (2, 0, 1, 1, 4200),
    (2, 1, 0, -1, -3359),
    (2, -1, -1, 1, 2463),
    (2, -1, 0, 1, 2211),
    (2, -1, -1, -1, 2065),
    (0, 1, -1, -1, -1870),
    (4, 0, -1, -1, 1828),
    (0, 1, 0, 1, -1794),
    (0, 0, 0, 3, -1749),
    (0, 1, -1, 1, -1565),
    (1, 0, 0, 1, -1491),
    (0, 1, 1, 1, -1475),
    (0, 1, 1, -1, -1410),
    (0, 1, 0, -1, -1344),
    (1, 0, 0, -1, -1335),
    (0, 0, 3, 1, 1107),
    (4, 0, 0, -1, 1021),
    (4, 0, -1, 1, 833),
    (0, 0, 1, -3, 777),
    (4, 0, -2, 1, 671),
    (2, 0, 0, -3, 607),
    (2, 0, 2, -1, 596),
    (2, -1, 1, -1, 491),
    (2, 0, -2, 1, -451),
    (0, 0, 3, -1, 439),
    (2, 0, 2, 1, 422),
    (2, 0, -3, -1, 421),
    (2, 1, -1, 1, -366),
    (2, 1, 0, 1, -351),
    (4, 0, 0, 1, 331),
    (2, -1, 1, 1, 315),
    (2, -2, 0, -1, 302),
    (0, 0, 1, 3, -283),
    (2, 1, 1, -1, -229),
    (1, 1, 0, -1, 223),
    (1, 1, 0, 1, 223),
    (0, 1, -2, -1, -220),
    (2, 1, -1, -1, -220),
    (1, 0, 1, 1, -185),
    (2, -1, -2, -1, 181),
    (0, 1, 2, 1, -177),
    (4, 0, -2, -1, 176),
    (4, -1, -1, -1, 166),
    (1, 0, 1, -1, -164),
    (4, 0, 1, -1, 132),
    (1, 0, -1, -1, -119),
    (4, -1, 0, -1, 115),
    (2, -2, 0, 1, 107),
];

/// Calculate the moon's mean longitude, eq (47.1).
/// In: Julian day in dynamical time
/// Out: Moon's mean longitude in degrees, [0, 360)
fn mean_longitude(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_longitude =
        218.3164477 + 481_267.88123421 * t - 0.0015786 * t2 + t3 / 538_841.0 - t4 / 65_194_000.0;

    let mapped = util::map_to_0_to_360(Degrees::new(mean_longitude));
    mapped
}

/// Calculate the moon's mean elongation, eq (47.2).
/// In: Julian day in dynamical time
/// Out: Moon's mean elongation in degrees, [0, 360)
fn mean_elongation(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_elongation =
        297.8501921 + 445_267.1114034 * t - 0.0018819 * t2 + t3 / 545_868.0 - t4 / 113_065_000.0;

    let mapped = util::map_to_0_to_360(Degrees::new(mean_elongation));
    mapped
}

/// Calculate the moon's mean anomaly, eq (47.4).
/// In: Julian day in dynamical time
/// Out: Moon's mean anomaly in degrees, [0, 360)
fn mean_anomaly(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_anomaly =
        134.9633964 + 477198.8675055 * t + 0.0087414 * t2 + t3 / 69_699.0 - t4 / 14_712_000.0;

    let mapped = util::map_to_0_to_360(Degrees::new(mean_anomaly));
    mapped
}

/// Calculate the moon's argument of latitude, eq (47.5).
/// In: Julian day in dynamical time
/// Out: Moon's argument of latitude in degrees, [0, 360)
fn argument_of_latitude(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let argument_of_latitude =
        93.2720950 + 483202.0175233 * t - 0.0036539 * t2 - t3 / 3_526_000.0 + t4 / 863_310_000.0;

    let mapped = util::map_to_0_to_360(Degrees::new(argument_of_latitude));
    mapped
}

/// Calculate the moon's longitude (lambda), page 342
/// In: Julian day in dynamical time
/// Out: Moon's longitude in degrees, [0, 360)
pub(crate) fn geocentric_longitude(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let a1 = Radians::from(util::map_to_0_to_360(Degrees::new(119.75 + 131.849 * t)));
    let a2 = Radians::from(util::map_to_0_to_360(Degrees::new(53.09 + 479264.290 * t)));

    let l_prime = Radians::from(mean_longitude(jd));
    let d = Radians::from(mean_elongation(jd));
    let m = Radians::from(sun::mean_anomaly(jd));
    let m_prime = Radians::from(mean_anomaly(jd));
    let f = Radians::from(argument_of_latitude(jd));
    let e = earth::eccentricity(jd);

    // SS: perturbation term for moon's longitude
    let mut sigma_l = SIGMA_L_AND_R_COEFFICIENTS.iter().fold(0.0, |accum, &c| {
        let sin_arg = c.0 as f64 * d.0 + c.1 as f64 * m.0 + c.2 as f64 * m_prime.0 + c.3 as f64 * f.0;
        let mut coeff = c.4 as f64;

        if c.1 != 0 {
            coeff *= e;
        }

        if c.1 == -2 || c.1 == 2 {
            coeff *= e;
        }

        let value = coeff * sin_arg.sin();
        accum + value
    });

    sigma_l += 3958.0 * a1.0.sin();
    sigma_l += 1962.0 * (l_prime - f).0.sin();
    sigma_l += 318.0 * a2.0.sin();

    let nutation_delta = nutation::nutation_in_longitude(jd);
    let l_prime_degrees = Degrees::from(l_prime);

    Degrees::new(l_prime_degrees.0 + sigma_l / 1_000_000.0 + Degrees::from(nutation_delta).0)
}

/// Calculate the moon's latitude (beta), page 342
/// In: Julian day in dynamical time
/// Out: Moon's latitude in degrees, [0, 360)
pub(crate) fn geocentric_latitude(jd: f64) -> Degrees {
    let t = jd::centuries_from_epoch_j2000(jd);

    let a1 = Radians::from(util::map_to_0_to_360(Degrees::new(119.75 + 131.849 * t)));
    let a3 = Radians::from(util::map_to_0_to_360(Degrees::new(
        313.45 + 481266.484 * t,
    )));

    let l_prime = Radians::from(mean_longitude(jd));
    let d = Radians::from(mean_elongation(jd));
    let m = Radians::from(sun::mean_anomaly(jd));
    let m_prime = Radians::from(mean_anomaly(jd));
    let f = Radians::from(argument_of_latitude(jd));
    let e = earth::eccentricity(jd);

    // SS: perturbation term for moon's latitude
    let mut sigma_b = SIGMA_B_COEFFICIENTS.iter().fold(0.0, |accum, &c| {
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

    sigma_b -= 2235.0 * l_prime.sin();
    sigma_b += 382.0 * a3.sin();
    sigma_b += 175.0 * (a1 - f).sin();
    sigma_b += 175.0 * (a1 + f).sin();
    sigma_b += 127.0 * (l_prime - m_prime).sin();
    sigma_b -= 115.0 * (l_prime + m_prime).sin();

    Degrees::new(sigma_b / 1_000_000.0)
}

/// Calculate the moon's distance (delta) from earth, page 342
/// In: Julian day in dynamical time
/// Out: Moon's distance from Earth, in kilometers
pub fn distance_from_earth(jd: f64) -> f64 {
    let d = Radians::from(mean_elongation(jd));
    let m = Radians::from(sun::mean_anomaly(jd));
    let m_prime = Radians::from(mean_anomaly(jd));
    let f = Radians::from(argument_of_latitude(jd));
    let e = earth::eccentricity(jd);

    // SS: perturbation term for moon's longitude
    let sigma_r = SIGMA_L_AND_R_COEFFICIENTS.iter().fold(0.0, |accum, &c| {
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
    use assert_approx_eq::assert_approx_eq;

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
        let mean_elongation = sun::mean_anomaly(jd);

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

    #[test]
    fn longitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let longitude = geocentric_longitude(jd);

        // Assert
        assert_approx_eq!(133.16726428105474, longitude, 0.000_001)
    }

    #[test]
    fn latitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let latitude = geocentric_latitude(jd);

        // Assert
        assert_approx_eq!(-3.229126, latitude, 0.000_001)
    }

    #[test]
    fn distance_test() {
        // SS: 1992 April 12, 0h TD
        let jd = jd::from_date(1992, 4, 12, 0.0);

        // Act
        let distance = distance_from_earth(jd);

        // Assert
        assert_approx_eq!(368_409.7, distance, 0.1)
    }
}
