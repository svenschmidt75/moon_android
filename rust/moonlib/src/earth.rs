//! Earth related calculations
//!
//! Siderial Day: Imagine a reference longitudinal half-circle at noon where the Sun is in transit, i.e.
//! the sun is at the zenith crossing the observer's meridian. Now the Earth keeps rotating around its
//! axis, but it also moves in its orbit around the sun. After Earth rotates by 360 degrees, the sun will
//! not be at the zenith again. This 360 degree "day" is called a siderial day, i.e. the stars are at the
//! same position as before.
//
// The Earth has to rotate more than 360 degrees for the sun to be at the zenith again. This is called a
// solar day.
//
// see https://www.youtube.com/watch?v=1wGFJd3j3ds
//
// The length of a solar day varies throughout the year, as the Earth moves around an eclipse, not a
// perfect circle. Siderial days are always the same length, as they are defined by Earth rotating
// once around its axis.
use crate::date::jd::JD;
use crate::ecliptic::true_obliquity;
use crate::nutation::nutation_in_longitude;
use crate::util::{degrees::Degrees, radians::Radians};

/// Calculate Earth's eccentricity, eq (47.6).
/// In: Julian day in dynamical time
pub fn eccentricity(jd: JD) -> f64 {
    let t = jd.centuries_from_epoch_j2000();
    let t2 = t * t;

    1.0 - 0.002516 * t - 0.0000074 * t2
}

/// Calculate the mean siderial time at Greenwich
/// Meeus, page 87, chapter 12
/// In: Julian Day
/// Out: Mean siderial time in degrees [0, 360)
pub(crate) fn mean_siderial_time(jd: JD) -> Degrees {
    let delta_jd = jd - JD::new(2_451_545.0);
    let t = delta_jd.jd / 36525.0;
    let t2 = t * t;
    let t3 = t * t2;
    let mean_siderial_time =
        280.46061836 + 360.98564736629 * delta_jd.jd + 0.000387933 * t2 - t3 / 38_710_000.0;
    Degrees(mean_siderial_time).map_to_0_to_360()
}

/// Calculate the apparent siderial time at Greenwich, which
/// takes Earth's nutation effects into account.
/// Meeus, page 87, chapter 12
/// In: Julian Day
/// Out: Mean siderial time in degrees [0, 360)
pub(crate) fn apparent_siderial_time(jd: JD) -> Degrees {
    let mean_siderial_time = mean_siderial_time(jd);
    let eps = true_obliquity(jd);
    let delta_psi = nutation_in_longitude(jd);

    let siderial_time = mean_siderial_time + Degrees::from(delta_psi) * Radians::from(eps).0.cos();
    siderial_time
}

/// Local siderial time
/// In:
/// siderial_time: Siderial time at Greenwich, either mean or apparent, in degrees [0, 360)
/// lambda_observer: Observer's longitude, in degrees [-180, 180)
/// (positive west, negative east of Greenwich)
/// Out:
/// Local siderial time
pub(crate) fn local_siderial_time(siderial_time: Degrees, longitude_observer: Degrees) -> Degrees {
    Degrees::new(siderial_time.0 - longitude_observer.0).map_to_0_to_360()
}

/// Calculate the local hour angle, which measures how far an object is from the observer's meridian,
/// measured westwards from south.
/// Said differently, an hour angle of 7h:21m means that this object passed the observer's meridian
/// 7h:21 minutes ago.
/// In:
/// siderial_time: Local siderial time (i.e. observer's siderial time), either mean or apparent, in degrees [0, 360)
/// right ascension: Right ascension of the object whose hour angle we calculate, in degrees [0, 360)
/// Out:
/// Hour angle
pub(crate) fn hour_angle(siderial_time: Degrees, right_ascension: Degrees) -> Degrees {
    Degrees::new(siderial_time.0 - right_ascension.0).map_to_0_to_360()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::date::Date;
    use crate::date::jd::JD;
    use crate::{coordinates, ecliptic};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn eccentricity_test() {
        // Arrange

        // SS: April 12th, 1992, 0h TD
        let jd = JD::new(2_448_724.5);

        // Act
        let e = eccentricity(jd);

        // Assert
        assert_approx_eq!(1.000194, e, 0.000001)
    }

    #[test]
    pub fn ecliptical_to_equatorial_test() {
        // Arrange
        let jd = JD::from_date(Date::new(1992, 4, 12.0));
        let longitude = Degrees::new(133.162655);
        let latitude = Degrees::new(-3.229126);
        let true_obliquity = ecliptic::true_obliquity(jd);

        // Act
        let (ra, dec) = coordinates::ecliptical_2_equatorial(longitude, latitude, true_obliquity);

        // Assert
        assert_approx_eq!(134.68392033025296, ra.0, 0.000_001);

        assert_eq!(8, ra.to_hms().0);
        assert_eq!(58, ra.to_hms().1);
        assert_approx_eq!(44.1408, ra.to_hms().2, 0.01);

        assert_approx_eq!(13.769657226951539, dec.0, 0.000_001);
    }
}
