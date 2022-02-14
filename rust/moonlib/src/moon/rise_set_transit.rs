//! Calculate rise, set and transit times for the moon

use crate::date::date::Date;
use crate::date::jd::JD;
use crate::moon::position::{geocentric_latitude, geocentric_longitude};
use crate::util::degrees::Degrees;
use crate::util::radians::Radians;
use crate::{constants, coordinates, earth, ecliptic};

pub enum Kind {
    Time(JD),
    NeverRises,
    NeverSets,
}

/// Compute the time the moon rises
/// In:
/// date: Date to compute the rise time for
/// longitude_observer: in degrees [-180, 180)
/// latitude_observer: in degrees, [-90, 90)
/// target_altitude: altitude of moon above horizon, in degrees [0, 90]
pub(crate) fn rise(
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    target_altitude: Degrees,
) -> Kind {
    // SS: initial time is noon
    let midday = Date::new(date.year, date.month, date.day.trunc() + 0.5);
    let base_jd = JD::from_date(midday);

    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();

    let target_altitude_radians = Radians::from(target_altitude);
    let sin_h0 = target_altitude_radians.0.sin();

    let mut iter = 0;
    const MAX_ITER: u8 = 10;

    let mut prev_jd = base_jd;

    loop {
        println!("Iteration {iter}: ");
        println!("-------------");

        println!("jd: {:.6}", prev_jd.jd);

        let date = prev_jd.to_calendar_date();
        let (h, m, s) = Date::from_fract_day(date.day);
        println!(
            "Date: {}/{}/{} {}:{}:{:.2}",
            date.year,
            date.month,
            date.day.trunc() as u8,
            h,
            m,
            s
        );

        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(prev_jd);
        println!("Longitude: {:.2}", longitude.0);

        let latitude = geocentric_latitude(prev_jd);
        println!("Latitude: {:.2}", latitude.0);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(prev_jd);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        let ra_hours = ra.to_hms();
        println!("RA: {}:{}:{:.2}", ra_hours.0, ra_hours.1, ra_hours.2);
        println!("Decl: {:.2}", decl.0);

        let decl_radians = Radians::from(decl);
        let sin_decl = decl_radians.0.sin();
        let cos_decl = decl_radians.0.cos();
        let cos_hour_angle =
            (sin_h0 - sin_latitude_observer * sin_decl) / (cos_latitude_observer * cos_decl);

        let hour_angle;

        if cos_hour_angle < -1.0 {
            return Kind::NeverRises;
        } else if cos_hour_angle > 1.0 {
            return Kind::NeverSets;
        } else {
            hour_angle = Degrees::from(Radians::new(cos_hour_angle.acos()));
        }

        let (azimuth, mut altitude) =
            coordinates::equatorial_2_horizontal(decl, hour_angle, latitude_observer);

        let hour_angle_hours = hour_angle.to_hms();

        let sin_h0_check = sin_decl * sin_latitude_observer
            + cos_latitude_observer * cos_decl * Radians::from(hour_angle).0.cos();
        println!("sin_h0: {:.8}", sin_h0);
        println!("sin_h0(tau1): {:.8}", sin_h0_check);
        println!(
            "sin(altitude(tau1)): {:.8}",
            Radians::from(altitude).0.sin()
        );

        // SS: calculate time correction from our angle
        let theta0 = earth::apparent_siderial_time(prev_jd);
        let theta = earth::local_siderial_time(theta0, longitude_observer);
        let theta_hours = theta.to_hms();
        println!(
            "theta: {}:{}:{:.2}",
            theta_hours.0, theta_hours.1, theta_hours.2
        );

        // SS: calculate hour angle at time jd2
        let hour_angle2 = (theta - ra).map_neg180_to_180();
        let hour_angle2_hours = hour_angle2.to_hms();

        println!("tau1: {:.2}", hour_angle.0);
        println!(
            "tau1: {}:{}:{:.2}",
            hour_angle_hours.0, hour_angle_hours.1, hour_angle_hours.2
        );

        println!("tau2: {:.2}", hour_angle2.0);
        println!(
            "tau2: {}:{}:{:.2}",
            hour_angle2_hours.0, hour_angle2_hours.1, hour_angle2_hours.2
        );

        // SS: + for rising time
        let delta_hour_angle = (hour_angle2 + hour_angle).map_neg180_to_180();

        // SS: convert degrees to time units
        let delta_t = delta_hour_angle.to_hours() * constants::SIDERIAL_TO_SOLAR_TIME;

        let delta_t_hours =
            Degrees::new(delta_hour_angle.0 * constants::SIDERIAL_TO_SOLAR_TIME).to_hms();
        println!(
            "delta t: {}:{}:{:.2}",
            delta_t_hours.0, delta_t_hours.1, delta_t_hours.2
        );

        let mut ojd = prev_jd;
        ojd.add_hours(-delta_t);
        let date = ojd.to_calendar_date();
        let (h, m, s) = Date::from_fract_day(date.day);
        println!(
            "Date: {}/{}/{} {}:{}:{:.2}",
            date.year,
            date.month,
            date.day.trunc() as u8,
            h,
            m,
            s
        );

        // SS: set new Julian Day
        prev_jd = ojd;

        if delta_t.abs() < 0.000008 || iter > MAX_ITER {
            break;
        }

        iter += 1;

        println!();
    }

    Kind::Time(prev_jd)
}

pub(crate) fn rise2(
    date: Date,
    longitude_observer: Degrees,
    latitude_observer: Degrees,
    target_altitude: Degrees,
) -> Kind {
    // SS: initial time is noon
    let midday = Date::new(date.year, date.month, date.day.trunc() + 0.5);
    let base_jd = JD::from_date(midday);
    //    let base_jd = JD::from_date(Date::from_date_hms(2000, 3, 23, 21, 14, 2.0));
    let mut offset_jd = JD::new(0.0);

    let latitude_observer_radians = Radians::from(latitude_observer);
    let sin_latitude_observer = latitude_observer_radians.0.sin();
    let cos_latitude_observer = latitude_observer_radians.0.cos();

    let target_altitude_radians = Radians::from(target_altitude);
    let sin_h0 = target_altitude_radians.0.sin();

    let mut iter = 0;
    const MAX_ITER: u8 = 10;

    let mut prev_jd = base_jd;

    loop {
        println!("Iteration {iter}: ");
        println!("-------------");

        // SS: ecliptical geocentric coordinates of the moon
        let longitude = geocentric_longitude(prev_jd);
        let latitude = geocentric_latitude(prev_jd);

        // SS: equatorial geocentric coordinates of the moon
        let eps = ecliptic::true_obliquity(prev_jd);
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);

        let ra_hours = ra.to_hms();
        println!("RA: {}:{}:{:.2}", ra_hours.0, ra_hours.1, ra_hours.2);

        let decl_radians = Radians::from(decl);
        let sin_decl = decl_radians.0.sin();
        let cos_decl = decl_radians.0.cos();
        let cos_hour_angle =
            (sin_h0 - sin_latitude_observer * sin_decl) / (cos_latitude_observer * cos_decl);

        let hour_angle;

        if cos_hour_angle < -1.0 {
            return Kind::NeverRises;
        } else if cos_hour_angle > 1.0 {
            return Kind::NeverSets;
        } else {
            hour_angle = Degrees::from(Radians::new(cos_hour_angle.acos()));
        }

        let (azimuth, mut altitude) =
            coordinates::equatorial_2_horizontal(decl, hour_angle, latitude_observer);

        let hour_angle_hours = hour_angle.to_hms();

        // SS: calculate time correction from our angle
        let theta0 = earth::apparent_siderial_time(prev_jd);
        let theta = earth::local_siderial_time(theta0, longitude_observer);
        let theta_hours = theta.to_hms();

        // SS: calculate hour angle at time jd2
        let hour_angle2 = (theta - ra).map_neg180_to_180();
        let hour_angle2_hours = hour_angle2.to_hms();

        let delta_hour_angle = (hour_angle2 + hour_angle).map_neg180_to_180();

        // SS: convert degrees to time units
        let delta_t = delta_hour_angle.to_hours() * constants::SIDERIAL_TO_SOLAR_TIME;

        let delta_t_hours =
            Degrees::new(delta_hour_angle.0 * constants::SIDERIAL_TO_SOLAR_TIME).to_hms();
        println!(
            "delta t: {}:{}:{:.2}",
            delta_t_hours.0, delta_t_hours.1, delta_t_hours.2
        );

        let ha = ra - theta; // + longitude_observer;

        // SS: - hour angle in time units
        let ut_moon_in_south = ha.0 / 15.04107;
        let ut_moon_rise = ut_moon_in_south - hour_angle.0 / 15.0;

        let mut ojd = prev_jd;
        ojd.add_hours(ut_moon_rise);
        let date = ojd.to_calendar_date();
        let (h, m, s) = Date::from_fract_day(date.day);
        println!(
            "Date: {}/{}/{} {}:{}:{:.2}",
            date.year,
            date.month,
            date.day.trunc() as u8,
            h,
            m,
            s
        );

        let dt = ojd - prev_jd;

        if dt.jd.abs() < 0.000008 || iter > MAX_ITER {
            let date = ojd.to_calendar_date();
            let (h, m, s) = Date::from_fract_day(date.day);
            println!(
                "Date: {}/{}/{} {}:{}:{:.2}",
                date.year,
                date.month,
                date.day.trunc() as u8,
                h,
                m,
                s
            );

            break;
        }

        iter += 1;

        //        offset_jd.add_hours(delta_t);

        prev_jd = ojd;

        println!();
    }

    Kind::Time(prev_jd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::date::Date;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn rise_test_1() {
        // Arrange
        let date = Date::new(2000, 3, 23.0);

        // SS: Munich, 11.6 deg east from Greenwich meridian
        let longitude_observer = Degrees::new(-11.6);

        let latitude_observer = Degrees::new(48.1);
        let target_altitude = Degrees::new(constants::MOON_SET_HEIGHT);

        // Act
        let k = rise(date, longitude_observer, latitude_observer, target_altitude);

        // Assert

        //        assert_approx_eq!(-180.0 + (d.0 - 180.0), angle.0, 0.000_001)
    }
}
