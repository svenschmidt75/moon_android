//! Calculate the moon's position for given Julian day.
//! see J. Meeus, Astronomical Algorithms, chapter 47
use crate::date::jd::JD;
use crate::util::{degrees::Degrees, radians::Radians};
use crate::{earth, nutation, sun::sun};
use tabular::moon_position_data;

/// Calculate the moon's mean longitude, eq (47.1).
/// In: Julian day in dynamical time
/// Out: Moon's mean longitude in degrees, [0, 360)
fn mean_longitude(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_longitude =
        218.3164477 + 481_267.88123421 * t - 0.0015786 * t2 + t3 / 538_841.0 - t4 / 65_194_000.0;

    Degrees::new(mean_longitude).map_to_0_to_360()
}

/// Calculate the moon's mean elongation, eq (47.2).
/// In: Julian day in dynamical time
/// Out: Moon's mean elongation in degrees, [0, 360)
fn mean_elongation(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_elongation =
        297.8501921 + 445_267.1114034 * t - 0.0018819 * t2 + t3 / 545_868.0 - t4 / 113_065_000.0;

    Degrees::new(mean_elongation).map_to_0_to_360()
}

/// Calculate the moon's mean anomaly, eq (47.4).
/// In: Julian day in dynamical time
/// Out: Moon's mean anomaly in degrees, [0, 360)
fn mean_anomaly(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let mean_anomaly =
        134.9633964 + 477198.8675055 * t + 0.0087414 * t2 + t3 / 69_699.0 - t4 / 14_712_000.0;

    Degrees::new(mean_anomaly).map_to_0_to_360()
}

/// Calculate the moon's argument of latitude, eq (47.5).
/// In: Julian day in dynamical time
/// Out: Moon's argument of latitude in degrees, [0, 360)
fn argument_of_latitude(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();

    let t2 = t * t;
    let t3 = t * t2;
    let t4 = t * t3;

    let argument_of_latitude =
        93.2720950 + 483202.0175233 * t - 0.0036539 * t2 - t3 / 3_526_000.0 + t4 / 863_310_000.0;

    Degrees::new(argument_of_latitude).map_to_0_to_360()
}

/// Calculate the moon's longitude (lambda), page 342
/// In: Julian day in dynamical time
/// Out: Moon's longitude in degrees, [0, 360)
pub(crate) fn geocentric_longitude(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();

    let a1 = Radians::from(Degrees::new(119.75 + 131.849 * t).map_to_0_to_360());
    let a2 = Radians::from(Degrees::new(53.09 + 479264.290 * t).map_to_0_to_360());

    let l_prime = Radians::from(mean_longitude(jd));
    let d = Radians::from(mean_elongation(jd));
    let m = Radians::from(sun::mean_anomaly(jd));
    let m_prime = Radians::from(mean_anomaly(jd));
    let f = Radians::from(argument_of_latitude(jd));
    let e = earth::eccentricity(jd);

    // SS: perturbation term for moon's longitude
    let mut sigma_l =
        moon_position_data::SIGMA_L_AND_R_COEFFICIENTS
            .iter()
            .fold(0.0, |accum, &c| {
                let sin_arg =
                    c.0 as f64 * d.0 + c.1 as f64 * m.0 + c.2 as f64 * m_prime.0 + c.3 as f64 * f.0;
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
pub(crate) fn geocentric_latitude(jd: JD) -> Degrees {
    let t = jd.centuries_from_epoch_j2000();

    let a1 = Radians::from(Degrees::new(119.75 + 131.849 * t).map_to_0_to_360());
    let a3 = Radians::from(Degrees::new(313.45 + 481266.484 * t).map_to_0_to_360());

    let l_prime = Radians::from(mean_longitude(jd));
    let d = Radians::from(mean_elongation(jd));
    let m = Radians::from(sun::mean_anomaly(jd));
    let m_prime = Radians::from(mean_anomaly(jd));
    let f = Radians::from(argument_of_latitude(jd));
    let e = earth::eccentricity(jd);

    // SS: perturbation term for moon's latitude
    let mut sigma_b = moon_position_data::SIGMA_B_COEFFICIENTS
        .iter()
        .fold(0.0, |accum, &c| {
            let sin_arg =
                c.0 as f64 * d.0 + c.1 as f64 * m.0 + c.2 as f64 * m_prime.0 + c.3 as f64 * f.0;
            let mut coeff = c.4 as f64;

            if c.1 != 0 {
                coeff *= e;
            }

            if c.1 == -2 || c.1 == 2 {
                coeff *= e;
            }

            accum + coeff * sin_arg.sin()
        });

    sigma_b -= 2235.0 * l_prime.0.sin();
    sigma_b += 382.0 * a3.0.sin();
    sigma_b += 175.0 * (a1 - f).0.sin();
    sigma_b += 175.0 * (a1 + f).0.sin();
    sigma_b += 127.0 * (l_prime - m_prime).0.sin();
    sigma_b -= 115.0 * (l_prime + m_prime).0.sin();

    Degrees::new(sigma_b / 1_000_000.0)
}

/// Calculate the moon's distance (delta) from earth, page 342
/// In: Julian day in dynamical time
/// Out: Moon's distance from Earth, in kilometers
pub fn distance_from_earth(jd: JD) -> f64 {
    let d = Radians::from(mean_elongation(jd));
    let m = Radians::from(sun::mean_anomaly(jd));
    let m_prime = Radians::from(mean_anomaly(jd));
    let f = Radians::from(argument_of_latitude(jd));
    let e = earth::eccentricity(jd);

    // SS: perturbation term for moon's longitude
    let sigma_r = moon_position_data::SIGMA_L_AND_R_COEFFICIENTS
        .iter()
        .fold(0.0, |accum, &c| {
            let cos_arg =
                c.0 as f64 * d.0 + c.1 as f64 * m.0 + c.2 as f64 * m_prime.0 + c.3 as f64 * f.0;
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
    use crate::date::date::Date;
    use crate::{coordinates, earth, ecliptic, refraction};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn mean_longitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let mean_longitude = mean_longitude(jd);

        // Assert
        assert_approx_eq!(134.290182, mean_longitude.0, 0.000_001)
    }

    #[test]
    fn mean_elongation_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let mean_elongation = mean_elongation(jd);

        // Assert
        assert_approx_eq!(113.842304, mean_elongation.0, 0.000_001)
    }

    #[test]
    fn sun_mean_anomaly_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let mean_elongation = sun::mean_anomaly(jd);

        // Assert
        assert_approx_eq!(97.643514, mean_elongation.0, 0.000_001)
    }

    #[test]
    fn mean_anomaly_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let mean_elongation = mean_anomaly(jd);

        // Assert
        assert_approx_eq!(5.150833, mean_elongation.0, 0.000_001)
    }

    #[test]
    fn argument_of_latitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let argument_of_latitude = argument_of_latitude(jd);

        // Assert
        assert_approx_eq!(219.889721, argument_of_latitude.0, 0.000_001)
    }

    #[test]
    fn longitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let longitude = geocentric_longitude(jd);

        // Assert
        assert_approx_eq!(133.16726428105474, longitude.0, 0.000_001)
    }

    #[test]
    fn latitude_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let latitude = geocentric_latitude(jd);

        // Assert
        assert_approx_eq!(-3.229126, latitude.0, 0.000_001)
    }

    #[test]
    fn distance_test() {
        // SS: 1992 April 12, 0h TD
        let jd = JD::from_date(Date::new(1992, 4, 12.0));

        // Act
        let distance = distance_from_earth(jd);

        // Assert
        assert_approx_eq!(368_409.7, distance, 0.1)
    }

    #[test]
    fn equatorial_2_topocentric_moon_test_1() {
        // Act
        let jd = JD::from_date(Date::from_date_hms(2003, 8, 28, 3, 17, 0.0));

        // SS: Mount Palomar
        let longitude_observer = Degrees::from_hms(7, 47, 27.0);
        let latitude_observer = Degrees::from_dms(33, 21, 22.0);
        let palomar_height_above_sea = 1706.0;

        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(jd);
        let latitude = geocentric_latitude(jd);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(jd);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        // SS: equatorial geocentric coordinates to equatorial topocentric coordinates
        let distance = distance_from_earth(jd);
        let (ra_topocentric_moon, decl_topocentric_moon) = coordinates::equatorial_2_topocentric(
            ra,
            decl,
            longitude_observer,
            latitude_observer,
            palomar_height_above_sea,
            distance,
            jd,
        );

        // SS: horizontal topocentric coordinates of the moon
        let siderial_time_apparent_greenwich = earth::apparent_siderial_time(jd);
        let siderial_time_local =
            earth::local_siderial_time(siderial_time_apparent_greenwich, longitude_observer);
        let hour_angle = earth::hour_angle(siderial_time_local, ra_topocentric_moon);
        let (azimuth, mut altitude) = coordinates::equatorial_2_horizontal(
            decl_topocentric_moon,
            hour_angle,
            latitude_observer,
        );

        // SS: add correction for atmospheric refraction
        let refraction_correction =
            refraction::refraction_from_apparent_altitude(altitude, 1013.0, 10.0);
        altitude += refraction_correction;

        // Assert
        assert_approx_eq!(108.74082230643148, azimuth.0, 0.000_1);
        assert_approx_eq!(-5.7132731871712839, altitude.0, 0.001);
    }

    #[test]
    fn equatorial_2_topocentric_moon_test_2() {
        // Act

        // SS: Sunday, Jan. 30th 2022, 1:55:57PM UTC
        let jd = JD::new(2_459_610.080526);

        // SS: Mount Palomar
        let longitude_observer = Degrees::from_hms(7, 47, 27.0);
        let latitude_observer = Degrees::from_dms(33, 21, 22.0);
        let palomar_height_above_sea = 1706.0;

        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(jd);
        let latitude = geocentric_latitude(jd);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(jd);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        // SS: equatorial geocentric coordinates to equatorial topocentric coordinates
        let distance = distance_from_earth(jd);
        let (ra_topocentric_moon, decl_topocentric_moon) = coordinates::equatorial_2_topocentric(
            ra,
            decl,
            longitude_observer,
            latitude_observer,
            palomar_height_above_sea,
            distance,
            jd,
        );

        // SS: horizontal topocentric coordinates of the moon
        let siderial_time_apparent_greenwich = earth::apparent_siderial_time(jd);
        let siderial_time_local =
            earth::local_siderial_time(siderial_time_apparent_greenwich, longitude_observer);
        let hour_angle = earth::hour_angle(siderial_time_local, ra_topocentric_moon);
        let (azimuth, mut altitude) = coordinates::equatorial_2_horizontal(
            decl_topocentric_moon,
            hour_angle,
            latitude_observer,
        );

        // SS: add correction for atmospheric refraction
        let refraction_correction =
            refraction::refraction_from_apparent_altitude(altitude, 1013.0, 10.0);
        altitude += refraction_correction;

        // Assert
        assert_approx_eq!(303.5642283477215, azimuth.0, 0.000_1);
        assert_approx_eq!(1.6965870451518825, altitude.0, 0.001);
    }
}
