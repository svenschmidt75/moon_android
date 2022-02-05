use crate::util::degrees::Degrees;
use crate::util::DEGREES_TO_RADIANS;
use std::ops::{Add, Sub};

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
