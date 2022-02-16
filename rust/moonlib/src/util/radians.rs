use crate::constants;
use crate::util::degrees::Degrees;
use std::ops::{Add, Mul, Sub};
use crate::util::arcsec::ArcSec;

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

impl Mul for Radians {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        let radians = degrees.0 * constants::DEGREES_TO_RADIANS;
        Self(radians)
    }
}

impl From<ArcSec> for Radians {
    fn from(arcsec: ArcSec) -> Self {
        let degrees = arcsec.0 / (60.0 * 60.0);
        let radians = degrees * constants::DEGREES_TO_RADIANS;
        Self(radians)
    }
}