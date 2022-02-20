//! Calculates the Moon's semidiameter

use crate::date::jd::JD;
use crate::moon::parallax::horizontal_equatorial_parallax;
use crate::parallax;
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

/// Calculates the geocentric semidiameter of the Moon
/// Meeus, chapter 55, page 390
/// In: Julian Day
/// Out: Moon's semidiameter in arcsec
fn geocentric_semidiameter(jd: JD) -> ArcSec {
    const K: f64 = 0.272_481;
    let sin_s = K * Radians::from(horizontal_equatorial_parallax(jd)).0;
    let s = sin_s.asin();
    ArcSec::from(Radians::new(s))
}

/// Calculate the Moon's topocentric semidiameter.
/// Meeus, chapter 55, page 391
/// In:
/// jd: Julian Day
/// hour_angle: observer's local our angle
/// decl: Moon's declination
/// latitude_observer: Observer's geocentric latitude
/// height: observer's height above sea level
/// Out:
/// Moon's semidiameter in degrees
pub(crate) fn topocentric_semidiameter(
    jd: JD,
    hour_angle: Degrees,
    decl: Degrees,
    latitude_observer: Degrees,
    height_observer: f64,
) -> ArcSec {
    let hour_angle_rad = Radians::from(hour_angle);
    let decl_rad = Radians::from(decl);

    let (rho_sin_p, rho_cos_p) = parallax::rho_phi_prime(latitude_observer, height_observer);

    // SS: eq. (40.7), page 280
    let sin_pi = Radians::from(horizontal_equatorial_parallax(jd));
    let a = decl_rad.0.cos() * hour_angle_rad.0.sin();
    let b = decl_rad.0.cos() * hour_angle_rad.0.cos() - rho_cos_p * sin_pi.0;
    let c = decl_rad.0.sin() - rho_sin_p * sin_pi.0;
    let q = (a * a + b * b + c * c).sqrt();

    let geocentric_semidiameter = Radians::from(geocentric_semidiameter(jd));
    let sin_sprime = Radians::new(geocentric_semidiameter.0.sin() / q);
    ArcSec::from(Radians::new(sin_sprime.0.asin()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::date::Date;
    use crate::date::jd::JD;
    use crate::moon::position::{geocentric_latitude, geocentric_longitude};
    use crate::{coordinates, ecliptic, moon};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn topocentric_semidiameter_test_1() {
        // Duffett-Smith, Peter and Zwart, Jonathan, Practical Astronomy with your Calculator
        // or Spreadsheet, 4th edition, page 176

        // Arrange
        let date = Date::new(1979, 9, 1.0);
        let jd = JD::from_date(date);

        let latitude_observer = Degrees::new(33.356111);
        let height_above_sea_level_observer = 1706.0;

        let longitude = geocentric_longitude(jd);
        let latitude = geocentric_latitude(jd);
        let eps = ecliptic::true_obliquity(jd);
        let (_, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        // Act
        let (d, m, s) = Degrees::from(topocentric_semidiameter(
            jd,
            Degrees::new(65.46),
            decl,
            latitude_observer,
            height_above_sea_level_observer,
        )).to_dms();

        // SS: calculated using Duffett-Smith, Peter and Zwart, Jonathan, Practical Astronomy with
        // your Calculator or Spreadsheet, 4th edition, page 176
        let theta0 = 0.5181;
        let a = 384_401.0;
        let distance = moon::position::distance_from_earth(jd);
        let rho_prime = distance / a;
        let (d2, m2, s2) = Degrees::new((theta0 / rho_prime) / 2.0).to_dms();

        // Assert
        assert_eq!(0, d);
        assert_eq!(0, d2);

        assert_eq!(15, m);
        assert_eq!(15, m2);

        assert_approx_eq!(51.611, s, 0.001);
        assert_approx_eq!(49.2075, s2, 0.001);
    }
}
