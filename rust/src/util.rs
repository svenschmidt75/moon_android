//! Utility functions

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;
const RADIANS_TO_DEGREES: f64 = 1.0 / DEGREES_TO_RADIANS;

#[derive(Debug, Clone, Copy)]
pub struct ArcSec(pub(crate) f64);

impl ArcSec {
    pub fn new(arcsec: f64) -> Self {
        Self(arcsec)
    }

    pub fn new_from_degrees(degrees: i16, minutes: i16, seconds: f64) -> Self {
        let arcsec = seconds + 60.0 * (minutes as f64 + 60.0 * degrees as f64);
        Self(arcsec)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Degrees(pub(crate) f64);

impl Degrees {
    pub fn new(degrees: f64) -> Self {
        Self(degrees)
    }

    pub fn to_dms(&self) -> (u8, u8, f64) {
        // SS: 360 degrees = 24 hrs
        let deg = self.0;

        let remainder = self.0 - deg.trunc();
        let minutes = remainder * 60.0;

        let remainder = minutes - minutes.trunc();
        let seconds = remainder * 60.0;

        (deg as u8, minutes as u8, seconds)
    }

    pub fn to_dms_str(&self) -> String {
        let (d, m, s) = self.to_dms();
        format!("{d}° {m}' {s:.2}\"")
    }

    pub fn to_hms(&self) -> (u8, u8, f64) {
        // SS: convert right ascension to h:m:s
        let h = self.0 / 360.0 * 24.0;

        let remainder = h - h.trunc();
        let m = remainder * 60.0;

        let remainder = m - m.trunc();
        let s = remainder * 60.0;

        (h.trunc() as u8, m.trunc() as u8, s)
    }

    pub fn to_hms_str(&self) -> String {
        let (h, m, s) = self.to_hms();
        format!("{h}h {m}m {s:.2}s")
    }

    /// Map angle in degrees to range [0, 360)
    pub fn map_to_0_to_360(self: Self) -> Self {
        let mut m = self.0 % 360.0;
        if m < 0.0 {
            m += 360.0;
        }
        Self::new(m)
    }

    pub fn map_to_neg90_to_90(self: Self) -> Self {
        Self(self.0 % 90.0)
    }
}

impl Add for Degrees {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Degrees {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Degrees {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Degrees {
    type Output = Degrees;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Neg for Degrees {
    type Output = Degrees;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Radians(pub(crate) f64);

impl Radians {
    pub fn new(radians: f64) -> Self {
        Self(radians)
    }
}

impl Add for Radians {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Radians {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        let radians = degrees.0 * DEGREES_TO_RADIANS;
        Self(radians)
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        let degrees = radians.0 * RADIANS_TO_DEGREES;
        Self(degrees)
    }
}

impl From<Degrees> for ArcSec {
    fn from(degrees: Degrees) -> Self {
        let degrees = degrees.0 * 3600.0;
        Self(degrees)
    }
}

impl From<ArcSec> for Degrees {
    fn from(arcsec: ArcSec) -> Self {
        // SS: 1 arcsec = 1/3600 degree
        let degrees = arcsec.0 / 3600.0;
        Self(degrees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn arcsec_to_degrees_test_1() {
        // Arrange
        let arcsec = ArcSec::new_from_degrees(133, 10, 2.154);

        // Act
        let degrees: f64 = Degrees::from(arcsec).0;

        // Assert
        assert_approx_eq!(133.167265, degrees, 0.000_1)
    }

    #[test]
    fn arcsec_to_degrees_test_2() {
        // Arrange
        let arcsec = ArcSec::new_from_degrees(23, 26, 26.29);

        // Act
        let degrees: f64 = Degrees::from(arcsec).0;

        // Assert
        assert_approx_eq!(23.440636, degrees, 0.000_001)
    }

    #[test]
    fn degree_to_dms_test() {
        // Arrange
        let degrees = Degrees(101.78654);

        // Act
        let text = format!("{}", degrees.to_dms_str());

        // Assert
        assert_eq!(r#"101° 47' 11.54""#, text)
    }

    #[test]
    fn degree_to_hms_test() {
        // Arrange
        let degrees = Degrees(101.78654);

        // Act
        let text = format!("{}", degrees.to_hms_str());

        // Assert
        assert_eq!("6h 47m 8.77s", text)
    }

    #[test]
    fn degrees_to_minutes_seconds_test() {
        // Arrange
        let angle = Degrees(133.167265);

        // Act
        let (d, m, s) = angle.to_dms();

        // Assert
        assert_eq!(133, d);
        assert_eq!(10, m);
        assert_approx_eq!(2.154, s, 0.001);
    }

    #[test]
    fn map_negative_1() {
        // Arrange
        let angle = Degrees(-10.0);

        // Act
        let mapped = angle.map_to_0_to_360();

        // Assert
        assert_eq!(360.0 + angle.0, mapped.0)
    }

    #[test]
    fn degrees_to_hours_test() {
        // Arrange
        let angle = Degrees(134.688470);

        // Act
        let (h, m, s) = angle.to_hms();

        // Assert
        assert_eq!(8, h);
        assert_eq!(58, m);
        assert_approx_eq!(45.2328, s, 0.001)
    }
}
