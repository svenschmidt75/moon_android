//! The Julian day is the number of days (and fractions thereof)  from the
//! beginning of the year -4712. This day corresponds to Greenwich mean
//! noon, i.e. 12h universal time.

/// Convert date to Julian day.
/// The date is assumed to be in dynamical time (TD). The correction
/// from DT to universal time (UT) is ignored, so for the purpose of this
/// module, TD = UT.
/// see J. Meeus, Astronomical Algorithms, chapter 7
pub fn from_date(y: i16, m: u8, d: f64) -> f64 {
    let (mm, yy) = if m < 3 { (m + 12, y - 1) } else { (m, y) };

    let b = if !is_julian_calendar(y, m, d) {
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
    jd
}

pub(crate) fn from_date_hms(year: i16, month: u8, day: u8, h: u8, m: u8, s: f64) -> f64 {
    let day_fraction = day as f64 + (h as f64 + (m as f64 + s / 60.0) / 60.0) / 24.0;
    from_date(year, month, day_fraction)
}

/// The Gregorian calendar reform implies that any date before
/// or at 1582, Oct. 4th is in the Julian calendar, dates after
/// in the Gregorian calendar.
fn is_julian_calendar(y: i16, m: u8, d: f64) -> bool {
    y < 1582 || y == 1582 && (m < 10 || m == 10 && d < 5.0)
}

pub fn centuries_from_epoch_j2000(jd: f64) -> f64 {
    // SS: Epoch J2000 = Jan. 1st, 2000 at 0hr UTC
    let jd_epoch_j2000 = 2_451_545.0;

    // SS: convert to dynamical time TD
    // 365.25 = 1 year => 36525 = 100 years
    let t = (jd - jd_epoch_j2000) / 36_525.0;
    t
}

pub fn millennia_from_epoch_j2000(jd: f64) -> f64 {
    // SS: Epoch J2000 = Jan. 1st, 2000 at 0hr UTC
    let jd_epoch_j2000 = 2_451_545.0;

    // SS: convert to dynamical time TD
    // 365.25 = 1 year => 365_250 = 1000 years = 1 millennium
    let t = (jd - jd_epoch_j2000) / 365_250.0;
    t
}

/// Convert Julian Day to Modified Julian Day MJD
/// Meeus, chapter 7, page 63
pub(crate) fn jd_to_mjd(jd: f64) -> f64 {
    // SS: Modified Julian Day = 0 corresponds to 1858 Nov. 17 at 0h UT
    jd - 2_400_000.5
}

/// Convert Modified Julian Day to Julian Day MJD
/// Meeus, chapter 7, page 63
pub fn mjd_to_jd(mjd: f64) -> f64 {
    // SS: Modified Julian Day = 0 corresponds to 1858 Nov. 17 at 0h UT
    mjd + 2_400_000.5
}

/// Convert fractional day to hh:mm:s
fn from_fract_day(day: f64) -> (u8, u8, f64) {
    let hours1 = 24.0 * day.fract();
    let hours = hours1.trunc();

    let minutes1 = (hours1 - hours) * 60.0;
    let minutes = minutes1.trunc();

    let seconds = (minutes1 - minutes) * 60.0;

    (hours as u8, minutes as u8, seconds)
}

/// Convert Julian Day to calendar date
/// Meeus, page 63, chapter 7
/// In: Julian Day
/// Out: Calendar date
pub(crate) fn to_calendar_date(jd: f64) -> (i16, u8, f64) {
    let jd_mod = jd + 0.5;
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

    (year as i16, m as u8, day_fract)
}

/// Determine whether year is a leap year
/// For Julian calendar dates, we check whether the year is divisible by 4.
/// For Gregorian calendar dates, see https://en.wikipedia.org/wiki/Leap_year
fn is_leap(y: i16) -> bool {
    return if is_julian_calendar(y, 1, 1.0) {
        y % 4 == 0
    } else {
        if y % 100 == 0 {
            y % 400 == 0
        } else {
            y % 4 == 0
        }
    };
}

/// Calculate the fractional year taking leap years into account
/// In: year, month, fractional day
/// Out: fractional year
pub(crate) fn fractional_year(y: i16, m: u8, d: f64) -> f64 {
    let days_in_year = if is_leap(y) { 366.0 } else { 365.0 };

    let jd = from_date(y, m, d);

    // SS: Julian Day at beginning of the same year
    let jd2 = from_date(y, 1, 1.0);

    y as f64 + (jd - jd2) / days_in_year
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn fractional_year_test() {
        // Arrange

        // Act
        let year_fract = fractional_year(2003, 8, 28.0);

        // Assert
        assert_approx_eq!(2003.654794520548, year_fract, 0.000_001);
    }

    #[test]
    fn is_leap_year_julian_calandar_test() {
        // Arrange
        let years = [900, 1236];

        for year in years {
            // Act
            let is_leap_year = is_leap(year);

            // Assert
            assert_eq!(true, is_leap_year);
        }
    }

    #[test]
    fn is_not_leap_year_julian_calandar_test() {
        // Arrange
        let years = [750, 1429];

        for year in years {
            // Act
            let is_leap_year = is_leap(year);

            // Assert
            assert_eq!(false, is_leap_year);
        }
    }

    #[test]
    fn is_leap_year_gregorian_calandar_test() {
        // Arrange
        let years = [1600, 2000, 2400];

        for year in years {
            // Act
            let is_leap_year = is_leap(year);

            // Assert
            assert_eq!(true, is_leap_year);
        }
    }

    #[test]
    fn is_not_leap_year_gregorian_calandar_test() {
        // Arrange
        let years = [1700, 1800, 1900, 2100];

        for year in years {
            // Act
            let is_leap_year = is_leap(year);

            // Assert
            assert_eq!(false, is_leap_year);
        }
    }

    #[test]
    fn from_fract_day_test() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let day_fract = 4.81;

        // Act
        let (hours, minutes, seconds) = from_fract_day(day_fract);

        // Assert
        assert_eq!(19, hours);
        assert_eq!(26, minutes);
        assert_approx_eq!(23.9999999, seconds, 0.000_001);
    }

    #[test]
    fn calendar_date_from_jd_test1() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let jd = 2_436_116.31;

        // Act
        let (year, month, day_fract) = to_calendar_date(jd);

        // Assert
        assert_eq!(1957, year);
        assert_eq!(10, month);
        assert_approx_eq!(4.81, day_fract, 0.001);
        assert_approx_eq!(jd, from_date(year, month, day_fract), 0.000_001);
    }

    #[test]
    fn calendar_date_from_jd_test2() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let jd = 1_842_713.0;

        // Act
        let (year, month, day_fract) = to_calendar_date(jd);

        // Assert
        assert_eq!(333, year);
        assert_eq!(1, month);
        assert_approx_eq!(27.5, day_fract, 0.001);
        assert_approx_eq!(jd, from_date(year, month, day_fract), 0.000_001);
    }

    #[test]
    fn calendar_date_from_jd_test3() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let jd = 1_507_900.13;

        // Act
        let (year, month, day_fract) = to_calendar_date(jd);

        // Assert
        assert_eq!(-584, year);
        assert_eq!(5, month);
        assert_approx_eq!(28.63, day_fract, 0.001);
        assert_approx_eq!(jd, from_date(year, month, day_fract), 0.000_001);
    }

    #[test]
    fn julian_date() {
        // arrange

        // act

        // assert
        assert!(is_julian_calendar(333, 1, 27.0))
    }

    #[test]
    fn gregorian_date() {
        // arrange

        // act

        // assert
        assert_ne!(true, is_julian_calendar(1957, 10, 4.0))
    }

    #[test]
    fn julian_day_gregorian_date() {
        // arrange

        // act

        // assert
        assert_eq!(2_436_116.31, from_date(1957, 10, 4.81))
    }

    #[test]
    fn julian_day_julian_date() {
        // arrange

        // act

        // assert
        assert_eq!(1_842_713.0, from_date(333, 1, 27.5))
    }

    #[test]
    fn meeus_1() {
        // Meeus, page 62

        // arrange

        // act

        // assert
        assert_eq!(0.0, from_date(-4712, 1, 1.5))
    }

    #[test]
    fn meeus_2() {
        // Meeus, page 62

        // arrange

        // act

        // assert
        assert_eq!(2_026_871.8, from_date(837, 4, 10.3))
    }

    #[test]
    fn julian_day_from_hms_test() {
        // arrange

        // act

        // 2003 August 28th, 3h:17m:0s UT
        let jd = from_date_hms(2003, 8, 28, 3, 17, 0.0);

        // assert
        assert_approx_eq!(2_452_879.63681, jd, 0.000_01)
    }
}
