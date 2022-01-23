//! Calculate corrections for parallax.

use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

/// Calculate the corrections needed to convert from geographical observer
/// latitude to the geocentric observer latitude.
/// Meeus, page 82, chapter 11
/// In: geographical latitude of the observer, in degrees [-90, 90)
/// height: Height of observer above sea level, in meters
/// Out: (rho * sin phi_p, rho * cos phi_p)
fn rho_phi_prime(latitude_geographical: Degrees, height: f64) -> (f64, f64) {
    let phi_p_radians = Radians::from(latitude_geographical);

    const B_OVER_A: f64 = 0.996_647_19;

    let u = (B_OVER_A * phi_p_radians.0.tan()).atan();

    let rho_sin_phi_p = B_OVER_A * u.sin() + height / (6_378_140.0) * phi_p_radians.0.sin();
    let rho_cos_phi_p =u.cos() + height / (6_378_140.0) * phi_p_radians.0.cos();

    (rho_sin_phi_p, rho_cos_phi_p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn rhO_phi_p_test() {
        // Meeus, page 82, example 11.a

        // Arrange
        let palomar_latitude = Degrees::from_dms(33, 21, 22.0);
        let palomar_height = 1706.0;

        // Act
        let (rho_sin_p, rho_cos_p) = rho_phi_prime(palomar_latitude, palomar_height);

        // Assert
        assert_approx_eq!(0.546_861, rho_sin_p, 0.000_001);
        assert_approx_eq!(0.836_339, rho_cos_p, 0.000_001);
    }
}
