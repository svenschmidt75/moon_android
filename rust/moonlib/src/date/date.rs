//! Functions for representing a date as year, month, fractional day

use crate::date::jd::JD;

#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub year: i16,
    pub month: u8,
    pub day: f64,
}

impl Date {
    pub(crate) fn new(year: i16, month: u8, day: f64) -> Self {
        Self { year, month, day }
    }

    pub(crate) fn from_date_hms(year: i16, month: u8, day: u8, h: u8, m: u8, s: f64) -> Date {
        let day_fraction = day as f64 + (h as f64 + (m as f64 + s / 60.0) / 60.0) / 24.0;
        Date::new(year, month, day_fraction)
    }

    /// Calculate the fractional year taking leap years into account
    /// In: year, month, fractional day
    /// Out: fractional year
    pub(crate) fn fractional_year(self) -> f64 {
        let days_in_year = if Date::is_leap(self.year) {
            366.0
        } else {
            365.0
        };

        let jd = JD::from_date(self);

        // SS: Julian Day at beginning of the same year
        let jd2 = JD::from_date(Date::new(self.year, 1, 1.0));

        self.year as f64 + (jd - jd2).jd / days_in_year
    }

    /// Determine whether year is a leap year
    /// For Julian calendar dates, we check whether the year is divisible by 4.
    /// For Gregorian calendar dates, see https://en.wikipedia.org/wiki/Leap_year
    fn is_leap(y: i16) -> bool {
        if Date::is_julian_calendar(Date::new(y, 1, 1.0)) {
            y % 4 == 0
        } else if y % 100 == 0 {
            y % 400 == 0
        } else {
            y % 4 == 0
        }
    }

    /// The Gregorian calendar reform implies that any date before
    /// or at 1582, Oct. 4th is in the Julian calendar, dates after
    /// in the Gregorian calendar.
    pub(crate) fn is_julian_calendar(self) -> bool {
        self.year < 1582
            || self.year == 1582 && (self.month < 10 || self.month == 10 && self.day < 5.0)
    }

    /// Convert fractional day to hh:mm:s
    pub(crate) fn from_fract_day(day: f64) -> (u8, u8, f64) {
        let hours1 = 24.0 * day.fract();
        let hours = hours1.trunc();

        let minutes1 = (hours1 - hours) * 60.0;
        let minutes = minutes1.trunc();

        let seconds = (minutes1 - minutes) * 60.0;

        (hours as u8, minutes as u8, seconds)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn fractional_year_test() {
        // Arrange
        let date = Date::new(2003, 8, 28.0);

        // Act
        let year_fract = date.fractional_year();

        // Assert
        assert_approx_eq!(2003.654794520548, year_fract, 0.000_001);
    }

    #[test]
    fn is_leap_year_julian_calandar_test() {
        // Arrange
        let years = [900, 1236];

        for year in years {
            // Act
            let is_leap_year = Date::is_leap(year);

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
            let is_leap_year = Date::is_leap(year);

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
            let is_leap_year = Date::is_leap(year);

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
            let is_leap_year = Date::is_leap(year);

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
        let (hours, minutes, seconds) = Date::from_fract_day(day_fract);

        // Assert
        assert_eq!(19, hours);
        assert_eq!(26, minutes);
        assert_approx_eq!(23.9999999, seconds, 0.000_001);
    }

    #[test]
    fn calendar_date_from_jd_test1() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let jd = JD::new(2_436_116.31);

        // Act
        let date = JD::to_calendar_date(jd);

        // Assert
        assert_eq!(1957, date.year);
        assert_eq!(10, date.month);
        assert_approx_eq!(4.81, date.day, 0.001);
        assert_approx_eq!(jd.jd, JD::from_date(date).jd, 0.000_001);
    }

    #[test]
    fn calendar_date_from_jd_test2() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let jd = JD::new(1_842_713.0);

        // Act
        let date = JD::to_calendar_date(jd);

        // Assert
        assert_eq!(333, date.year);
        assert_eq!(1, date.month);
        assert_approx_eq!(27.5, date.day, 0.001);
        assert_approx_eq!(jd.jd, JD::from_date(date).jd, 0.000_001);
    }

    #[test]
    fn calendar_date_from_jd_test3() {
        // SS: Example 7.c, page 64, chapter 7, Meeus

        // Arrange
        let jd = JD::new(1_507_900.13);

        // Act
        let date = JD::to_calendar_date(jd);

        // Assert
        assert_eq!(-584, date.year);
        assert_eq!(5, date.month);
        assert_approx_eq!(28.63, date.day, 0.001);
        assert_approx_eq!(jd.jd, JD::from_date(date).jd, 0.000_001);
    }

    #[test]
    fn julian_date() {
        // arrange
        let date = Date::new(333, 1, 27.0);

        // act

        // assert
        assert!(date.is_julian_calendar())
    }

    #[test]
    fn gregorian_date() {
        // arrange
        let date = Date::new(1957, 10, 4.0);

        // act

        // assert
        assert_ne!(true, date.is_julian_calendar())
    }
}
