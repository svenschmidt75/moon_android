//! Moon's parallax

use crate::{constants};
use crate::date::jd::JD;
use crate::moon::position::distance_from_earth;
use crate::util::arcsec::ArcSec;
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;

/// Calculate the Moon's equatorial horizontal parallax.
/// Meeus, chapter 47, page 337
/// In: Julian Day
/// Out: horizontal parallax, in degrees
pub(crate) fn horizontal_equatorial_parallax(jd: JD) -> ArcSec {
    let distance = distance_from_earth(jd);
    ArcSec::from(Radians::new(constants::EARTH_RADIUS / distance))
}

/// Calculate the Moon's horizontal parallax.
/// Meeus, chapter 40, page 281
/// In: Julian Day
/// altitude: altitude, in degrees [-90, 90)
/// Out: horizontal parallax, in arcsecs
fn horizontal_parallax(jd: JD, altitude: Degrees) -> ArcSec {
    let altitude_rad = Radians::from(altitude);

    let sin_pi = horizontal_equatorial_parallax(jd);
    let sin_p = sin_pi.0 * altitude_rad.0.cos();
    ArcSec::from(Radians::new(sin_p.asin()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use crate::date::date::Date;
    use crate::date::jd::JD;
    use crate::moon::position::distance_from_earth;

    #[test]
    fn horizontal_parallax_test_1() {
        // Duffett-Smith, Peter and Zwart, Jonathan, Practical Astronomy with your Calculator
        // or Spreadsheet, 4th edition, page 176

        // Arrange
        let date = Date::new(1979, 9, 1.0);
        let jd = JD::from_date(date);

        let distance = distance_from_earth(jd);

        // Act
        let hor_parallax = horizontal_parallax(jd, Degrees::new(0.0));

        // Assert
        assert_approx_eq!(Degrees::from_dms(1, 0, 0.12).0, Degrees::from(hor_parallax).0, 0.033);
    }
}
