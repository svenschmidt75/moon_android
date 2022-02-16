//! Moon's parallax

use crate::parallax;
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

fn parallax_altitude(distance_delta: f64, altitude: Degrees, latitude_observer: Degrees, height_observer: f64) -> ArcSec {
    let altitude_rad = Radians::from(altitude);
    let latitude_observer_rad = Degrees::from(latitude_observer);

    let (rho_sin_p, rho_cos_p) = parallax::rho_phi_prime(latitude_observer, height_observer);
    let rho = rho_sin_p / latitude_observer_rad.0.sin();

    let sin_pi = (6378.14 / distance_delta).sin();

    let parallax = rho * sin_pi * altitude_rad.0.cos();
    ArcSec::from(Radians::new(parallax.asin()))
}