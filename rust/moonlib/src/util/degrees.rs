//! Utility functions

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use crate::constants;
use crate::util::arcsec::ArcSec;
use crate::util::radians::Radians;

#[derive(Debug, Clone, Copy)]
pub struct Degrees(pub(crate) f64);

impl Degrees {
    pub fn new(degrees: f64) -> Self {
        Self(degrees)
    }

    pub fn from_dms(d: i16, m: u8, s: f64) -> Self {
        let sign = if d < 0 { -1.0 } else { 1.0 };
        let value = sign * d as f64 + (m as f64 + s / 60.0) / 60.0;
        Self(sign * value)
    }

    pub fn from_hms(h: u8, m: u8, s: f64) -> Self {
        // SS: note that h >= 0
        let f = 360.0 / 24.0;
        Self(f * (h as f64 + (m as f64 + s / 60.0) / 60.0))
    }

    pub fn to_dms(&self) -> (i16, u8, f64) {
        // SS: 360 degrees = 24 hrs

        let sign = if self.0 < 0.0 { -1 } else { 1 };

        let degress = self.0.abs();

        let deg = degress;

        let remainder = degress - deg.trunc();
        let minutes = remainder * 60.0;

        let remainder = minutes - minutes.trunc();
        let seconds = remainder * 60.0;

        (sign * deg as i16, minutes as u8, seconds)
    }

    pub fn to_dms_str(&self, width: u8) -> String {
        let (d, m, s) = self.to_dms();
        format!("{d}° {m}' {s:.width$}\"", width = width as usize)
    }

    pub fn to_hms(&self) -> (i8, u8, f64) {
        // SS: convert right ascension to h:m:s
        let sign = if self.0 < 0.0 { -1 } else { 1};
        let h = self.0.abs() / 360.0 * 24.0;

        let remainder = h - h.trunc();
        let m = remainder * 60.0;

        let remainder = m - m.trunc();
        let s = remainder * 60.0;

        (sign * h.trunc() as i8, m.trunc() as u8, s)
    }

    pub fn to_hours(&self) -> f64 {
        // SS: convert right ascension to fractional hours
        let h = self.0 * constants::DEGREES_TO_HOURS;
        h
    }

    pub fn to_hms_str(&self, width: u8) -> String {
        let (h, m, s) = self.to_hms();
        format!("{h}h {m}m {s:.width$}s", width = width as usize)
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

    /// Convert angle range
    /// In: angle in degrees, [0..360)
    /// Out: angle, in degrees [-180, 180)
    pub fn map_neg180_to_180(self: Self) -> Self {
        return if self.0 < -180.0 {
            let tmp = self.0 % 180.0;
            Self(180.0 + tmp)
        } else if self.0 > 180.0 {
            let tmp = self.0 % 180.0;
            Self(-180.0 + tmp)
        } else {
            self
        };
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

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        let degrees = radians.0 * constants::RADIANS_TO_DEGREES;
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
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn map_neg180_to_180_test1() {
        // Arrange
        let d = Degrees::new(189.0);

        // Act
        let angle = d.map_neg180_to_180();

        // Assert
        assert_approx_eq!(-180.0 + (d.0 - 180.0), angle.0, 0.000_001)
    }

    #[test]
    fn map_neg180_to_180_test2() {
        // Arrange
        let d = Degrees::new(-189.0);

        // Act
        let angle = d.map_neg180_to_180();

        // Assert
        assert_approx_eq!(180.0 + (d.0 + 180.0), angle.0, 0.000_001)
    }

    #[test]
    fn map_neg180_to_180_test3() {
        // Arrange
        let d = Degrees::new(89.0);

        // Act
        let angle = d.map_neg180_to_180();

        // Assert
        assert_approx_eq!(d.0, angle.0, 0.000_001)
    }

    #[test]
    fn arcsec_to_degrees_test_1() {
        // Arrange
        let arcsec = ArcSec::from_dms(133, 10, 2.154);

        // Act
        let degrees: f64 = Degrees::from(arcsec).0;

        // Assert
        assert_approx_eq!(133.167265, degrees, 0.000_1)
    }

    #[test]
    fn arcsec_to_degrees_test_2() {
        // Arrange
        let arcsec = ArcSec::from_dms(23, 26, 26.29);

        // Act
        let degrees: f64 = Degrees::from(arcsec).0;

        // Assert
        assert_approx_eq!(23.440636, degrees, 0.000_001)
    }

    #[test]
    fn degree_to_dms_test_1() {
        // Arrange
        let degrees = Degrees::new(13.769657226951539);

        // Act
        let text = format!("{}", degrees.to_dms_str(2));

        // Assert
        assert_eq!(r#"13° 46' 10.77""#, text)
    }

    #[test]
    fn degree_to_dms_test_2() {
        // Arrange
        let degrees = Degrees::new(-19.6475);

        // Act
        let text = format!("{}", degrees.to_dms_str(2));

        // Assert
        assert_eq!(r#"-19° 38' 51.00""#, text)
    }

    #[test]
    fn degree_to_hms_test_1() {
        // Arrange
        let degrees = Degrees::new(134.68392033025296);

        // Act
        let text = format!("{}", degrees.to_hms_str(2));

        // Assert
        assert_eq!("8h 58m 44.14s", text)
    }

    #[test]
    fn degree_to_hms_test_2() {
        // Arrange
        let degrees = Degrees::new(-137.817).map_to_0_to_360();

        // Act
        let (h, m, s) = degrees.to_hms();

        // Assert
        assert_eq!(14, h);
        assert_eq!(48, m);
        assert_approx_eq!(43.92, s, 0.001);
    }

    #[test]
    fn degrees_to_hms_test_3() {
        // Arrange
        let angle = Degrees::new(134.688470);

        // Act
        let (h, m, s) = angle.to_hms();

        // Assert
        assert_eq!(8, h);
        assert_eq!(58, m);
        assert_approx_eq!(45.2328, s, 0.001)
    }

    #[test]
    fn degrees_to_hms_test_4() {
        // Arrange
        let angle = Degrees::new(-130.94010921668462);

        // Act
        let (h, m, s) = angle.to_hms();

        // Assert
        assert_eq!(-8, h);
        assert_eq!(43, m);
        assert_approx_eq!(45.6262, s, 0.000_1)
    }

    #[test]
    fn degrees_2_dms_test() {
        // Arrange
        let angle = Degrees::new(133.167265);

        // Act
        let (d, m, s) = angle.to_dms();

        // Assert
        assert_eq!(133, d);
        assert_eq!(10, m);
        assert_approx_eq!(2.154, s, 0.000_001);
    }

    #[test]
    fn dms_2_degrees_test_1() {
        // Arrange

        // Act
        let angle = Degrees::from_dms(133, 10, 2.154);

        // Assert
        assert_approx_eq!(133.167265, angle.0, 0.001);
    }

    #[test]
    fn dms_2_degrees_test_2() {
        // Arrange

        // Act
        let angle = Degrees::from_dms(-6, 43, 11.61);

        // Assert
        assert_approx_eq!(-6.71989, angle.0, 0.000_1);
    }

    #[test]
    fn map_negative_1() {
        // Arrange
        let angle = Degrees::new(-10.0);

        // Act
        let mapped = angle.map_to_0_to_360();

        // Assert
        assert_eq!(360.0 + angle.0, mapped.0)
    }

    #[test]
    fn hms_2_degrees_test() {
        // Arrange

        // Act
        let angle = Degrees::from_hms(8, 58, 45.2328);

        // Assert
        assert_approx_eq!(134.688470, angle.0, 0.000_001)
    }

    #[test]
    fn degrees_to_hours_test_1() {
        // Arrange
        let angle = Degrees::new(360.0);

        // Act
        let hours = angle.to_hours();

        // Assert
        assert_approx_eq!(24.0, hours, 0.000_001)
    }

    #[test]
    fn degrees_to_hours_test_2() {
        // Arrange
        let angle = Degrees::new(360.0 / 2.0);

        // Act
        let hours = angle.to_hours();

        // Assert
        assert_approx_eq!(24.0 / 2.0, hours, 0.000_001)
    }
}
