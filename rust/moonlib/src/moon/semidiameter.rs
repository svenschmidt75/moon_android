//! Calculates the Moon's semidiameter

use crate::parallax;
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

/// Calculates the geocentric semidiameter of the Moon
/// Meeus, chapter 55, page 390
/// In: distance_delta: Distance Earth - Moon (from the center of both)
/// Out: Moon's semidiameter in arcsec
fn geocentric_semidiameter(distance_delta: f64) -> ArcSec {
    const K: f64 = 0.272_481;
    let sin_s = K * (6378.14 / distance_delta).sin();
    let s = sin_s.asin();
    ArcSec::new(s)
}

/// Calculate the Moon's topocentric semidiameter.
/// Meeus, chapter 55, page 390
/// In:
/// distance_delta: Distance Earth - Moon (from the center of both)
/// hour_angle: observer's local our angle
/// decl: Moon's declination
/// latitude_observer: Observer's geocentric latitude
/// height: observer's height above sea level
/// Out:
/// Moon's semidiameter in arcsec
pub(crate) fn topocentric_semidiameter(
    distance_delta: f64,
    hour_angle: Degrees,
    decl: Degrees,
    latitude_observer: Degrees,
    height_observer: f64,
) -> ArcSec {
    let hour_angle_rad = Radians::from(hour_angle);
    let decl_rad = Radians::from(decl);

    let geocentric_semidiameter = geocentric_semidiameter(distance_delta);

    let (rho_sin_p, rho_cos_p) = parallax::rho_phi_prime(latitude_observer, height_observer);

    // SS: eq. (40.7), page 280
    let sin_pi = (6378.14 / distance_delta).sin();
    let a = decl_rad.0.cos() * hour_angle_rad.0.sin();
    let b = decl_rad.0.cos() * hour_angle_rad.0.cos() - rho_cos_p * sin_pi;
    let c = decl_rad.0.sin() - rho_sin_p * sin_pi;
    let q = (a * a + b * b + c * c).sqrt();

    let sin_sprime = Radians::from(ArcSec::new(geocentric_semidiameter.0.sin() / q));
    let sprime = ArcSec::from(Radians::new(sin_sprime.0.asin()));
    sprime
}
