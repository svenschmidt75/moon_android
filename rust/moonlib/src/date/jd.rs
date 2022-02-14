//! Functions for representing a date as Julian Day
use crate::constants;
use crate::date::date::Date;

#[derive(Debug, Copy, Clone)]
pub struct JD {
    pub jd: f64,
}

impl JD {
    pub fn new(jd: f64) -> Self {
        Self { jd }
    }

    /// Convert date to Julian day.
    /// The date is assumed to be in dynamical time (TD). The correction
    /// from DT to universal time (UT) is ignored, so for the purpose of this
    /// module, TD = UT.
    /// see J. Meeus, Astronomical Algorithms, chapter 7
    pub(crate) fn from_date(date: Date) -> Self {
        let y = date.year;
        let m = date.month;
        let d = date.day;

        let (mm, yy) = if m < 3 { (m + 12, y - 1) } else { (m, y) };

        let b = if !Date::is_julian_calendar(date) {
            let a = (yy as f64 / 100.0).trunc();
            2.0 - a + (a as f64 / 4.0).trunc()
        } else {
            0.0
        };

        let jd = (365.25 * (yy as f64 + 4716.0)).trunc()
            + (30.6001 * (mm as f64 + 1.0)).trunc()
            + (d as f64)
            + b
            - 1524.5;
        Self { jd }
    }

    /// Convert Modified Julian Day to Julian Day MJD
    /// Meeus, chapter 7, page 63
    pub fn from_mjd(mjd: f64) -> Self {
        // SS: Modified Julian Day = 0 corresponds to 1858 Nov. 17 at 0h UT
        Self {
            jd: mjd + constants::MJD,
        }
    }

    pub(crate) fn centuries_from_epoch_j2000(self) -> f64 {
        // SS: convert to dynamical time TD
        // 365.25 = 1 year => 36525 = 100 years
        let t = (self.jd - constants::J2000) / 36_525.0;
        t
    }

    pub(crate) fn millennia_from_epoch_j2000(self) -> f64 {
        // SS: convert to dynamical time TD
        // 365.25 = 1 year => 365_250 = 1000 years = 1 millennium
        let t = (self.jd - constants::J2000) / 365_250.0;
        t
    }

    /// Convert Julian Day to Modified Julian Day MJD
    /// Meeus, chapter 7, page 63
    pub(crate) fn to_mjd(self) -> Self {
        // SS: Modified Julian Day = 0 corresponds to 1858 Nov. 17 at 0h UT
        Self {
            jd: self.jd - constants::MJD,
        }
    }

    /// Convert Julian Day to calendar date
    /// Meeus, page 63, chapter 7
    /// In: Julian Day
    /// Out: Calendar date
    pub fn to_calendar_date(self) -> Date {
        let jd_mod = self.jd + 0.5;
        let z = jd_mod.trunc();
        let f = jd_mod - z;

        let a = if z < 2_299_161.0 {
            z
        } else {
            let alpha = ((z - 1_867_216.25) / 36_524.25).trunc();
            z + 1.0 + alpha - (alpha / 4.0).trunc()
        };

        let b = a + 1524.0;
        let c = ((b - 122.1) / 365.25).trunc();
        let d = (365.25 * c).trunc();
        let e = ((b - d) / 30.6001).trunc();

        let day_fract = b - d - (30.6001 * e).trunc() + f;
        let m = if e < 14.0 { e - 1.0 } else { e - 13.0 };
        let year = if m > 2.0 { c - 4716.0 } else { c - 4715.0 };

        Date::new(year as i16, m as u8, day_fract)
    }

    /// Add delta_t days to Julian Day
    pub(crate) fn add_hours(&mut self, delta_t: f64) {
        // SS: the unit of a Julian day is days, so convert hours to days
        let days = delta_t * constants::HOURS_TO_DAYS;
        self.jd += days;
    }
}

impl std::ops::Add for JD {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            // SS: in units of (fractions of) days
            jd: self.jd + other.jd,
        }
    }
}

impl std::ops::Sub for JD {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            // SS: in units of (fractions of) days
            jd: self.jd - other.jd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn julian_day_gregorian_date() {
        // arrange
        let date = Date::new(1957, 10, 4.81);

        // act

        // assert
        assert_eq!(2_436_116.31, JD::from_date(date).jd)
    }

    #[test]
    fn julian_day_julian_date() {
        // arrange
        let date = Date::new(333, 1, 27.5);

        // act

        // assert
        assert_eq!(1_842_713.0, JD::from_date(date).jd)
    }

    #[test]
    fn meeus_1() {
        // Meeus, page 62
        let date = Date::new(-4712, 1, 1.5);

        // arrange

        // act

        // assert
        assert_eq!(0.0, JD::from_date(date).jd)
    }

    #[test]
    fn meeus_2() {
        // Meeus, page 62

        // arrange
        let date = Date::new(837, 4, 10.3);

        // act

        // assert
        assert_eq!(2_026_871.8, JD::from_date(date).jd)
    }

    #[test]
    fn julian_day_from_hms_test() {
        // arrange
        let date = Date::from_date_hms(2003, 8, 28, 3, 17, 0.0);
        // act

        // 2003 August 28th, 3h:17m:0s UT
        let jd = JD::from_date(date);

        // assert
        assert_approx_eq!(2_452_879.63681, jd.jd, 0.000_01)
    }

    #[test]
    fn add_hours_test_1() {
        // arrange
        let date = Date::new(2000, 1, 1.5);
        let mut jd = JD::from_date(date);

        // act
        jd.add_hours(12.0);

        // assert
        assert_approx_eq!(constants::J2000 + 0.5, jd.jd, 0.000_01)
    }

    #[test]
    fn add_hours_test_2() {
        // arrange
        let date = Date::new(2000, 3, 23.5);
        let mut jd = JD::from_date(date);

        // act
        jd.add_hours(4.809);

        // assert
        assert_approx_eq!(JD::from_date(Date::from_date_hms(2000, 3, 23, 16, 48, 32.7)).jd, jd.jd, 0.000_01)
    }
}
