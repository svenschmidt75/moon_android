use crate::nutation::nutation_in_longitude;
use crate::sun::vsop87d_ear;
use crate::util::{ArcSec, Degrees, Radians};
use crate::{jd, util};

/// Astronomical unit, in km
const AU: f64 = 149_597_870.0;

/// Calculate the heliocentric ecliptical longitude using the VSOP87
/// theory. Meeus, chapter 32, eq. (32.2)
/// In: Julian day
/// Out: Longitude in degrees [0, 360)
pub fn heliocentric_ecliptical_longitude(jd: f64) -> Degrees {
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

    util::map_to_0_to_360(Degrees::from(Radians::new(total_sum)))
}

/// Calculate the heliocentril ecliptical latitude using the VSOP87
/// theory. Meeus, chapter 32, eq. (32.2)
/// In: Julian day
/// Out: Latitude in degrees [0, 360)
pub fn heliocentric_ecliptical_latitude(jd: f64) -> Degrees {
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
    util::map_to_neg90_to_90(Degrees::from(Radians::new(total_sum)))
}

/// Calculate the distance Earth-Sun using the VSOP87
/// theory. Meeus, chapter 32, eq. (32.2)
/// In: Julian day
/// Out: Distance of the Earth to the sun in km
pub fn distance_earth_sun(jd: f64) -> f64 {
    let distance_ae = distance_earth_sun_ae(jd);
    distance_ae * AU
}

/// Calculate the distance Earth-Sun using the VSOP87
/// theory. Meeus, chapter 32, eq. (32.2)
/// In: Julian day
/// Out: Distance of the Earth, in AU
pub fn distance_earth_sun_ae(jd: f64) -> f64 {
    let millennia_from_j2000 = jd::millennia_from_epoch_j2000(jd);

    let mut total_sum = 0.0;
    let mut tau = 1.0;
    for (coeff, _) in vsop87d_ear::VSOP87D_R_EARTH {
        let mut sum = 0.0;

        for &(a, b, c) in coeff.iter() {
            let local_sum = a * (b + c * millennia_from_j2000).cos();
            sum += local_sum;
        }

        total_sum += sum * tau;
        tau *= millennia_from_j2000;
    }

    total_sum
}

/// Calculate the geocentric ecliptical longitude
/// Meeus, chapter 25, page 166
/// In: heliocentric ecliptical longitude in degrees [0, 360)
/// Out: geocentric ecliptical longitude in degrees [0, 360)
pub fn geocentric_ecliptical_longitude(jd: f64) -> Degrees {
    let heliocentric_ecliptical_longitude = heliocentric_ecliptical_longitude(jd);
    util::map_to_0_to_360(heliocentric_ecliptical_longitude + Degrees::new(180.0))
}

/// Calculate the geocentric ecliptical latitude
/// Meeus, chapter 25, page 166
/// In: heliocentric ecliptical latitude in degrees [-90, 90)
/// Out: geocentric ecliptical latitude in degrees [-90, 90)
pub fn geocentric_ecliptical_latitude(jd: f64) -> Degrees {
    let heliocentric_ecliptical_latitude = heliocentric_ecliptical_latitude(jd);
    -heliocentric_ecliptical_latitude
}

/// Calculate the geocentric ecliptical longitude and latitude in the FK5
/// system. Meeus, chapter 32, page 219, eq. (32.3)
/// In: geocentric ecliptical longitude in degrees [0, 360), from VSOP87
/// In: geocentric ecliptical longitude in degrees [-90, 90), from VSOP87
/// Out: geocentric ecliptical longitude in degrees [0, 360), corrected for FK5, w.r.t. mean equinox of the date
/// Out: geocentric ecliptical latitude in degrees [-90, 90), corrected for FK5, w.r.t. mean equinox of the date
pub fn geocentric_ecliptical_to_fk5(
    jd: f64,
    longitude: Degrees,
    latitude: Degrees,
) -> (Degrees, Degrees) {
    let mut ecliptical_longitude = longitude;
    let mut ecliptical_latitude = latitude;

    let centuries_from_j2000 = jd::centuries_from_epoch_j2000(jd);
    let lambda_prime = ecliptical_longitude.0
        - 1.397 * centuries_from_j2000
        - 0.000_31 * centuries_from_j2000 * centuries_from_j2000;
    let lambda_prime = Radians::from(util::map_to_0_to_360(Degrees::new(lambda_prime)));

    let delta_longitude = -0.09033
        + 0.03916 * (lambda_prime.0.cos() + lambda_prime.0.sin()) * Radians::from(latitude).0.tan();
    let delta_longitude = Degrees::from(ArcSec::new(delta_longitude));
    ecliptical_longitude += delta_longitude;

    let delta_latitude = 0.03916 * (lambda_prime.0.cos() - lambda_prime.0.sin());
    let delta_latitude = Degrees::from(ArcSec::new(delta_latitude));
    ecliptical_latitude += delta_latitude;

    (ecliptical_longitude, ecliptical_latitude)
}

/// Daily variation of the geocentric longitude. Meeus chapter 25,
/// page 168.
/// In: Julian day
/// Out: variation, in arcsec
fn variation_geocentric_longitude(jd: f64) -> ArcSec {
    let tau = jd::millennia_from_epoch_j2000(jd);
    let tau2 = tau * tau;
    let tau3 = tau2 * tau;

    let delta_lambda = 3548.193
        + 118.568 * Radians::new(87.5287 + 359993.7286 * tau).0.sin()
        + 2.476 * Radians::new(85.0561 + 719987.4571 * tau).0.sin()
        + 1.376 * Radians::new(27.8502 + 4452671.1152 * tau).0.sin()
        + 0.119 * Radians::new(73.1375 + 450368.8564 * tau).0.sin()
        + 0.114 * Radians::new(337.2264 + 329644.6718 * tau).0.sin()
        + 0.086 * Radians::new(222.5400 + 659289.3436 * tau).0.sin()
        + 0.078 * Radians::new(162.8136 + 9224659.7915 * tau).0.sin()
        + 0.054 * Radians::new(82.5823 + 1079981.1857 * tau).0.sin()
        + 0.052 * Radians::new(171.5189 + 225184.4282 * tau).0.sin()
        + 0.034 * Radians::new(30.3214 + 4092677.3866 * tau).0.sin()
        + 0.033 * Radians::new(119.8105 + 337181.4711 * tau).0.sin()
        + 0.023 * Radians::new(247.5418 + 299295.6151 * tau).0.sin()
        + 0.023 * Radians::new(325.1526 + 315559.5560 * tau).0.sin()
        + 0.021 * Radians::new(155.1241 + 675553.2846 * tau).0.sin()
        + 7.311 * tau * Radians::new(333.4515 + 359993.7286 * tau).0.sin()
        + 0.305 * tau * Radians::new(330.9814 + 719987.4571 * tau).0.sin()
        + 0.010 * tau * Radians::new(328.5170 + 1079981.1857 * tau).0.sin()
        + 0.309 * tau2 * Radians::new(241.4518 + 359993.7286 * tau).0.sin()
        + 0.021 * tau2 * Radians::new(205.0482 + 719987.4571 * tau).0.sin()
        + 0.004 * tau2 * Radians::new(297.8610 + 4452671.1152 * tau).0.sin()
        + 0.010 * tau3 * Radians::new(154.7066 + 359993.7286 * tau).0.sin();

    ArcSec::new(delta_lambda)
}

/// Calculate the corrections in geocentric longitude of the sun due to
/// both nutation and aberration. Meeus, chapter 25, pages 167, 168
/// In: Julian day
/// Out: Apparent geocentric longitude of the sun, in degrees [0, 360)
pub fn apparent_geometric_longitude(jd: f64) -> Degrees {
    let longitude = geocentric_ecliptical_longitude(jd);
    let latitude = geocentric_ecliptical_latitude(jd);
    let (long, _) = geocentric_ecliptical_to_fk5(jd, longitude, latitude);

    let r = distance_earth_sun_ae(jd);

    // SS: correction due to nutation
    let delta_psi = Degrees::from(nutation_in_longitude(jd));

    let delta_lambda = Degrees::from(variation_geocentric_longitude(jd));
    let aberration_correction = delta_lambda * (-0.005_775_518 * r);

    Degrees::from(util::map_to_0_to_360(
        long + delta_psi + aberration_correction,
    ))
}

/// Apparent geocentric latitude of the sun. Meeus, chapter 25, pages 167, 168
/// In: Julian day
/// Out: Apparent geocentric latitude of the sun, in degrees [-90, 90)
pub fn apparent_geometric_latitude(jd: f64) -> Degrees {
    let longitude = geocentric_ecliptical_longitude(jd);
    let latitude = geocentric_ecliptical_latitude(jd);
    let (_, lat) = geocentric_ecliptical_to_fk5(jd, longitude, latitude);
    util::map_to_neg90_to_90(lat)
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
        assert_approx_eq!(19.907, longitude.0, 0.001)
    }

    #[test]
    fn geocentric_ecliptical_longitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let longitude = geocentric_ecliptical_longitude(jd);

        // Assert
        assert_approx_eq!(199.907, longitude.0, 0.001)
    }

    #[test]
    fn distance_earth_sun_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let distance = distance_earth_sun_ae(jd);

        // Assert
        assert_approx_eq!(0.9976085202355933, distance, 0.000_001)
    }

    #[test]
    fn heliocentric_ecliptical_latitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let latitude = heliocentric_ecliptical_latitude(jd);

        // Assert
        assert_approx_eq!(-0.00020664594475074705, latitude.0, 0.001)
    }

    #[test]
    fn geocentric_ecliptical_latitude_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let latitude = geocentric_ecliptical_latitude(jd);

        // Assert
        assert_approx_eq!(0.00020664594475074705, latitude.0, 0.001)
    }

    #[test]
    fn geocentric_ecliptical_to_fk5_test() {
        // SS: 1992 October 13, 0h TD
        let jd = jd::from_date(1992, 10, 13, 0.0);

        // Act
        let longitude = apparent_geometric_longitude(jd);

        // Assert
        assert_approx_eq!(199.90598818016153, longitude.0, 0.000_001);
    }
}
