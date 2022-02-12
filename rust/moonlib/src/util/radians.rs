use crate::util::degrees::Degrees;
use std::ops::{Add, Mul, Sub};
use crate::constants;

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

// pub trait Mul<Rhs = Self> {
//     312 | |     /// The resulting type after applying the `*` operator.
//     313 | |     #[stable(feature = "rust1", since = "1.0.0")]
//     314 | |     type Output;
//     ...   |
//     325 | |     fn mul(self, rhs: Rhs) -> Self::Output;
//     326 | | }

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
