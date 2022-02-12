/// Astronomical unit, in km
pub(crate) const AU: f64 = 149_597_870.700;

pub(crate) const SEC_PER_DAY: i32 = 24 * 60 * 60;

/// 0:0:0.00 UT on November 17, 1858
pub(crate) const MJD: f64 = 2_400_000.5;

/// 12:0:0.00 UT on January 1, 2000
pub(crate) const J2000: f64 = 2_451_545.0;

/// Convert siderial time to solar time
pub(crate) const SIDERIAL_TO_SOLAR_TIME: f64 = 1.0;

/// Convert degrees to radians
pub(crate) const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;

/// Convert radians to degrees
pub(crate) const RADIANS_TO_DEGREES: f64 = 1.0 / DEGREES_TO_RADIANS;

/// Convert degrees to fractional hours
pub(crate) const DEGREES_TO_HOURS: f64 = 24.0 / 360.0;

/// Convert hours to days
pub(crate) const HOURS_TO_DAYS: f64 = 1.0 / 24.0;

/// Moon's height above the horizon when it sets
pub(crate) const MOON_SET_HEIGHT: f64 = -0.583;
