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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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
