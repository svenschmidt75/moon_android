pub(crate) mod arcsec;
pub(crate) mod binary_search;
pub(crate) mod degrees;
pub(crate) mod radians;

const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;
const RADIANS_TO_DEGREES: f64 = 1.0 / DEGREES_TO_RADIANS;
