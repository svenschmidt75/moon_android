use crate::util::degrees::Degrees;

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

impl From<Degrees> for ArcSec {
    fn from(degrees: Degrees) -> Self {
        let degrees = degrees.0 * 3600.0;
        Self(degrees)
    }
}
