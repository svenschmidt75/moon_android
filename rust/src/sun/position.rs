use crate::sun::vsop87d_ear;
use crate::util::ArcSec;
use crate::{jd, util};

/// Calculate the heliocentril ecliptical longitude using the VSOP87
/// theory. Meeus, chapter 32, eq. (32.2)
/// In: Julian day
/// Out: Longitude in degrees [0, 360)
pub fn heliocentric_ecliptical_longitude(jd: f64) -> f64 {
    let millennia_from_j2000 = jd::millennia_from_epoch_j2000(jd);

    let mut total_sum = 0.0;
    let mut tau = 1.0;
    for (coeff, _) in vsop87d_ear::VSOP87D_L_EARTH {
        let mut sum = 0.0;

        for &(a, b, c) in coeff.iter() {
            let local_sum = a * (b + c * millennia_from_j2000).cos();
            sum += local_sum;
        }

        total_sum += sum * tau;
        tau *= millennia_from_j2000;
    }

    util::map_to_0_to_360(util::to_degrees(total_sum))
}

/// Calculate the heliocentril ecliptical latitude using the VSOP87
/// theory. Meeus, chapter 32, eq. (32.2)
/// In: Julian day
/// Out: Latitude in degrees [0, 360)
pub fn heliocentric_ecliptical_latitude(jd: f64) -> f64 {
    let millennia_from_j2000 = jd::millennia_from_epoch_j2000(jd);

    let mut total_sum = 0.0;
    let mut tau = 1.0;
    for (coeff, _) in vsop87d_ear::VSOP87D_B_EARTH {
        let mut sum = 0.0;

        for &(a, b, c) in coeff.iter() {
            let local_sum = a * (b + c * millennia_from_j2000).cos();
            sum += local_sum;
        }

        total_sum += sum * tau;
        tau *= millennia_from_j2000;
    }

    // SS: latitude is defined for [-90, 90]
    util::map_to_neg90_to_90(util::to_degrees(total_sum))
}

/// Calculate the geocentric ecliptical longitude
/// Meeus, chapter 25, page 166
/// In: heliocentric ecliptical longitude in degrees [0, 360)
/// Out: geocentric ecliptical longitude in degrees [0, 360)
pub fn geocentric_ecliptical_longitude(jd: f64) -> f64 {
    let heliocentric_ecliptical_longitude = heliocentric_ecliptical_longitude(jd);
    util::map_to_0_to_360(heliocentric_ecliptical_longitude + 180.0)
}

/// Calculate the geocentric ecliptical latitude
/// Meeus, chapter 25, page 166
/// In: heliocentric ecliptical latitude in degrees [-90, 90)
/// Out: geocentric ecliptical latitude in degrees [-90, 90)
pub fn geocentric_ecliptical_latitude(jd: f64) -> f64 {
    let heliocentric_ecliptical_latitude = heliocentric_ecliptical_latitude(jd);
    -heliocentric_ecliptical_latitude
}

/// Calculate the geocentric ecliptical longitude and latitude in the FK5
/// system. Meeus, chapter 25, page 166
/// In: geocentric ecliptical longitude in degrees [0, 360), from VSOP87
/// In: geocentric ecliptical longitude in degrees [-90, 90), from VSOP87
/// Out: geocentric ecliptical longitude in degrees [0, 360), corrected for FK5, w.r.t. mean equinox of the date
/// Out: geocentric ecliptical latitude in degrees [-90, 90), corrected for FK5, w.r.t. mean equinox of the date
pub fn geocentric_ecliptical_to_fk5(jd: f64, longitude: f64, latitude: f64) -> (f64, f64) {
    let mut ecliptical_longitude = longitude;
    let mut ecliptical_latitude = latitude;

    let centuries_from_j2000 = jd::centuries_from_epoch_j2000(jd);
    let mut lambda_prime = ecliptical_longitude
        - 1.397 * centuries_from_j2000
        - 0.000_31 * centuries_from_j2000 * centuries_from_j2000;
    lambda_prime = util::to_radians(util::map_to_0_to_360(lambda_prime));

    let longitude_correction = ArcSec {
        degrees: 0,
        minutes: 0,
        seconds: 0.09033,
    };
    let delta_longitude = f64::from(longitude_correction);
    ecliptical_longitude += delta_longitude;

    let latitude_correction = ArcSec {
        degrees: 0,
        minutes: 0,
        seconds: 0.03916,
    };
    let delta_latitude = f64::from(latitude_correction) * (lambda_prime.cos() - lambda_prime.sin());
    ecliptical_latitude += delta_latitude;

    (ecliptical_longitude, ecliptical_latitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn heliocentric_ecliptical_longitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let longitude = heliocentric_ecliptical_longitude(jd);

        // Assert
        assert_approx_eq!(19.907, longitude, 0.001)
    }

    #[test]
    fn geocentric_ecliptical_longitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let longitude = geocentric_ecliptical_longitude(jd);

        // Assert
        assert_approx_eq!(199.907, longitude, 0.001)
    }

    #[test]
    fn heliocentric_ecliptical_latitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let latitude = heliocentric_ecliptical_latitude(jd);

        // Assert
        assert_approx_eq!(-0.00020664594475074705, latitude, 0.001)
    }

    #[test]
    fn geocentric_ecliptical_latitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let latitude = geocentric_ecliptical_latitude(jd);

        // Assert
        assert_approx_eq!(0.00020664594475074705, latitude, 0.001)
    }

    #[test]
    fn geocentric_ecliptical_to_fk5_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);
        let longitude = geocentric_ecliptical_longitude(jd);
        let latitude = geocentric_ecliptical_latitude(jd);

        // Act
        let (long, lat) = geocentric_ecliptical_to_fk5(jd, longitude, latitude);

        // Assert
        assert_approx_eq!(199.90732233371614, long, 0.000_001);
        assert_approx_eq!(0.00020014657618539123, lat, 0.000_001);
    }
}
