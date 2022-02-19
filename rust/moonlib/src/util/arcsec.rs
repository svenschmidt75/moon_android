use crate::util::degrees::Degrees;
use crate::util::radians::Radians;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct ArcSec(pub(crate) f64);

impl ArcSec {
    pub fn new(arcsec: f64) -> Self {
        Self(arcsec)
    }

    pub fn from_dms(degrees: i16, minutes: i16, seconds: f64) -> Self {
        let arcsec = seconds + 60.0 * (minutes as f64 + 60.0 * degrees as f64);
        Self(arcsec)
    }
}

impl Add for ArcSec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for ArcSec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for ArcSec {
    type Output = ArcSec;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<Degrees> for ArcSec {
    fn from(degrees: Degrees) -> Self {
        let arcsecs = degrees.0 * 3600.0;
        Self(arcsecs)
    }
}

impl From<Radians> for ArcSec {
    fn from(radians: Radians) -> Self {
        let degrees = Degrees::from(radians);
        ArcSec::from(degrees)
    }
}
