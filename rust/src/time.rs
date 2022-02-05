//! Time-related function.
//!
//! Siderial Day: Imagine a reference longitudinal half-circle at noon where the Sun is in transit, i.e.
//! the sun is at the zenith crossing the observer's meridian. Now the Earth keeps rotating around its
//! axis, but it also moves in its orbit around the sun. After Earth rotates by 360 degrees, the sun will
//! not be at the zenith again. This 360 degree "day" is called a siderial day, i.e. the stars are at the
//! same position as before.
use std::cmp::Ordering;
//
// The Earth has to rotate more than 360 degrees for the sun to be at the zenith again. This is called a
// solar day.
//
// see https://www.youtube.com/watch?v=1wGFJd3j3ds
//
// The length of a solar day varies throughout the year, as the Earth moves around an eclipse, not a
// perfect circle. Siderial days are always the same length, as they are defined by Earth rotating
// once around its axis.
use crate::ecliptic::true_obliquity;
use crate::nutation::nutation_in_longitude;
use crate::util::{degrees::Degrees, radians::Radians};
use crate::{jd, util};

/// Calculate the mean siderial time at Greenwich
/// Meeus, page 87, chapter 12
/// In: Julian Day
/// Out: Mean siderial time in degrees [0, 360)
pub fn mean_siderial_time(jd: f64) -> Degrees {
    let delta_jd = jd - 2_451_545.0;
    let t = delta_jd / 36525.0;
    let t2 = t * t;
    let t3 = t * t2;
    let mean_siderial_time =
        280.46061836 + 360.98564736629 * delta_jd + 0.000387933 * t2 - t3 / 38_710_000.0;
    Degrees(mean_siderial_time).map_to_0_to_360()
}

/// Calculate the apparent siderial time at Greenwich, which
/// takes Earth's nutation effects into account.
/// Meeus, page 87, chapter 12
/// In: Julian Day
/// Out: Mean siderial time in degrees [0, 360)
pub fn apparent_siderial_time(jd: f64) -> Degrees {
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

// MOVE INTO SEPARATE CRATE

struct LeapSecondCoefficient {
    jd: f64,
    leap_seconds: f64,
    base_mjd: f64,
    coefficient: f64,
}

impl PartialEq<Self> for LeapSecondCoefficient {
    fn eq(&self, other: &Self) -> bool {
        self.jd == other.jd
    }
}

impl Eq for LeapSecondCoefficient {}

impl PartialOrd<Self> for LeapSecondCoefficient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.jd < other.jd {
            Some(Ordering::Less)
        } else if self.jd > other.jd {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

/// Implement total ordering so we can use binary search.
/// Note that we only care about the Julian Day field, jd,
/// which is well-defined (never NaN)
impl std::cmp::Ord for LeapSecondCoefficient {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Data based on https://cddis.nasa.gov/archive/products/iers/tai-utc.dat
/// This table needs to be updated every few years to take new data into
/// account.
const LEAP_SECOND_DATA: [LeapSecondCoefficient; 41] = [
    LeapSecondCoefficient {
        jd: 2437300.5, // 1 Jan 1961
        leap_seconds: 1.4228180,
        base_mjd: 37300.0,
        coefficient: 0.001296,
    },
    LeapSecondCoefficient {
        jd: 2437512.5,
        leap_seconds: 1.3728180,
        base_mjd: 37300.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2437665.5,
        leap_seconds: 1.8458580,
        base_mjd: 37665.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438334.5,
        leap_seconds: 1.9458580,
        base_mjd: 37665.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438395.5,
        leap_seconds: 3.2401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438486.5,
        leap_seconds: 3.3401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438639.5,
        leap_seconds: 3.4401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438761.5,
        leap_seconds: 3.5401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438820.5,
        leap_seconds: 3.6401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2438942.5,
        leap_seconds: 3.7401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2439004.5,
        leap_seconds: 3.8401300,
        base_mjd: 38761.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2439126.5,
        leap_seconds: 4.3131700,
        base_mjd: 39126.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2439887.5,
        leap_seconds: 4.2131700,
        base_mjd: 39126.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2441317.5,
        leap_seconds: 10.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2441499.5,
        leap_seconds: 11.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2441683.5,
        leap_seconds: 12.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2442048.5,
        leap_seconds: 13.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2442413.5,
        leap_seconds: 14.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2442778.5,
        leap_seconds: 15.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2443144.5,
        leap_seconds: 16.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2443509.5,
        leap_seconds: 17.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2443874.5,
        leap_seconds: 18.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2444239.5,
        leap_seconds: 19.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2444786.5,
        leap_seconds: 20.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2445151.5,
        leap_seconds: 21.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2445516.5,
        leap_seconds: 22.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2446247.5,
        leap_seconds: 23.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2447161.5,
        leap_seconds: 24.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2447892.5,
        leap_seconds: 25.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2448257.5,
        leap_seconds: 26.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2448804.5,
        leap_seconds: 27.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2449169.5,
        leap_seconds: 28.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2449534.5,
        leap_seconds: 29.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2450083.5,
        leap_seconds: 30.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2450630.5,
        leap_seconds: 31.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2451179.5,
        leap_seconds: 32.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2453736.5,
        leap_seconds: 33.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2454832.5,
        leap_seconds: 34.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2456109.5,
        leap_seconds: 35.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2457204.5,
        leap_seconds: 36.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
    LeapSecondCoefficient {
        jd: 2457754.5, // 1 January 2017
        leap_seconds: 37.0,
        base_mjd: 41317.0,
        coefficient: 0.0,
    },
];

/// Calculate the amount of leap seconds for the date passed in.
/// This is to calculate TAI from UTC, i.e. TAI - UTC = cumulative_leap_seconds(UTC)
/// In: Julian Day, in UTC
/// Out: cumulative leap seconds for input date
pub(crate) fn cumulative_leap_seconds(jd: f64) -> f64 {
    let mut cumulative_leap_secs = 0.0;

    let mut idx = LEAP_SECOND_DATA.len() - 1;

    if jd >= LEAP_SECOND_DATA[0].jd {
        if jd < LEAP_SECOND_DATA[idx].jd {
            let to_find = LeapSecondCoefficient {
                jd,
                leap_seconds: 0.0,
                base_mjd: 0.0,
                coefficient: 0.0,
            };
            idx = util::binary_search::upper_bound(&LEAP_SECOND_DATA, &to_find);
        }

        let leap_item = &LEAP_SECOND_DATA[idx - 1];
        cumulative_leap_secs = leap_item.leap_seconds
            + (jd::jd_to_mjd(jd) - leap_item.base_mjd) * leap_item.coefficient;
    }

    cumulative_leap_secs
}

struct DeltaTValue {
    jd: f64,
    delta_t: f64,
}

const DELTA_T_DATA: [DeltaTValue; 632] = [
    // SS: historical data is based on https://cddis.nasa.gov/archive/products/iers/historic_deltat.data
    DeltaTValue {
        jd: 2326267.50,
        delta_t: 44.000,
    }, // Year 1657.000
    DeltaTValue{jd: 2326450.00, delta_t: 43.000 }, // Year 1657.50
    DeltaTValue{jd: 2326632.50, delta_t: 43.000 }, // Year 1658.00
    DeltaTValue{jd: 2326815.00, delta_t: 41.000 }, // Year 1658.50
    DeltaTValue{jd: 2326997.50, delta_t: 40.000 }, // Year 1659.00
    DeltaTValue{jd: 2327180.00, delta_t: 39.000 }, // Year 1659.50
    DeltaTValue{jd: 2327362.50, delta_t: 38.000 }, // Year 1660.00
    DeltaTValue{jd: 2327545.50, delta_t: 37.000 }, // Year 1660.50
    DeltaTValue{jd: 2327728.50, delta_t: 37.000 }, // Year 1661.00
    DeltaTValue{jd: 2327911.00, delta_t: 36.000 }, // Year 1661.50
    DeltaTValue{jd: 2328093.50, delta_t: 36.000 }, // Year 1662.00
    DeltaTValue{jd: 2328276.00, delta_t: 36.000 }, // Year 1662.50
    DeltaTValue{jd: 2328458.50, delta_t: 37.000 }, // Year 1663.00
    DeltaTValue{jd: 2328641.00, delta_t: 37.000 }, // Year 1663.50
    DeltaTValue{jd: 2328823.50, delta_t: 38.000 }, // Year 1664.00
    DeltaTValue{jd: 2329006.50, delta_t: 37.000 }, // Year 1664.50
    DeltaTValue{jd: 2329189.50, delta_t: 36.000 }, // Year 1665.00
    DeltaTValue{jd: 2329372.00, delta_t: 36.000 }, // Year 1665.50
    DeltaTValue{jd: 2329554.50, delta_t: 35.000 }, // Year 1666.00
    DeltaTValue{jd: 2329737.00, delta_t: 35.000 }, // Year 1666.50
    DeltaTValue{jd: 2329919.50, delta_t: 34.000 }, // Year 1667.00
    DeltaTValue{jd: 2330102.00, delta_t: 33.000 }, // Year 1667.50
    DeltaTValue{jd: 2330284.50, delta_t: 33.000 }, // Year 1668.00
    DeltaTValue{jd: 2330467.50, delta_t: 32.000 }, // Year 1668.50
    DeltaTValue{jd: 2330650.50, delta_t: 32.000 }, // Year 1669.00
    DeltaTValue{jd: 2330833.00, delta_t: 31.000 }, // Year 1669.50
    DeltaTValue{jd: 2331015.50, delta_t: 31.000 }, // Year 1670.00
    DeltaTValue{jd: 2331198.00, delta_t: 30.000 }, // Year 1670.50
    DeltaTValue{jd: 2331380.50, delta_t: 30.000 }, // Year 1671.00
    DeltaTValue{jd: 2331563.00, delta_t: 29.000 }, // Year 1671.50
    DeltaTValue{jd: 2331745.50, delta_t: 29.000 }, // Year 1672.00
    DeltaTValue{jd: 2331928.50, delta_t: 29.000 }, // Year 1672.50
    DeltaTValue{jd: 2332111.50, delta_t: 29.000 }, // Year 1673.00
    DeltaTValue{jd: 2332294.00, delta_t: 29.000 }, // Year 1673.50
    DeltaTValue{jd: 2332476.50, delta_t: 28.000 }, // Year 1674.00
    DeltaTValue{jd: 2332659.00, delta_t: 28.000 }, // Year 1674.50
    DeltaTValue{jd: 2332841.50, delta_t: 27.000 }, // Year 1675.00
    DeltaTValue{jd: 2333024.00, delta_t: 27.000 }, // Year 1675.50
    DeltaTValue{jd: 2333206.50, delta_t: 26.000 }, // Year 1676.00
    DeltaTValue{jd: 2333389.50, delta_t: 26.000 }, // Year 1676.50
    DeltaTValue{jd: 2333572.50, delta_t: 25.000 }, // Year 1677.00
    DeltaTValue{jd: 2333755.00, delta_t: 25.000 }, // Year 1677.50
    DeltaTValue{jd: 2333937.50, delta_t: 25.000 }, // Year 1678.00
    DeltaTValue{jd: 2334120.00, delta_t: 26.000 }, // Year 1678.50
    DeltaTValue{jd: 2334302.50, delta_t: 26.000 }, // Year 1679.00
    DeltaTValue{jd: 2334485.00, delta_t: 26.000 }, // Year 1679.50
    DeltaTValue{jd: 2334667.50, delta_t: 26.000 }, // Year 1680.00
    DeltaTValue{jd: 2334850.50, delta_t: 25.000 }, // Year 1680.50
    DeltaTValue{jd: 2335033.50, delta_t: 25.000 }, // Year 1681.00
    DeltaTValue{jd: 2335216.00, delta_t: 25.000 }, // Year 1681.50
    DeltaTValue{jd: 2335398.50, delta_t: 24.000 }, // Year 1682.00
    DeltaTValue{jd: 2335581.00, delta_t: 24.000 }, // Year 1682.50
    DeltaTValue{jd: 2335763.50, delta_t: 24.000 }, // Year 1683.00
    DeltaTValue{jd: 2335946.00, delta_t: 24.000 }, // Year 1683.50
    DeltaTValue{jd: 2336128.50, delta_t: 24.000 }, // Year 1684.00
    DeltaTValue{jd: 2336311.50, delta_t: 24.000 }, // Year 1684.50
    DeltaTValue{jd: 2336494.50, delta_t: 24.000 }, // Year 1685.00
    DeltaTValue{jd: 2336677.00, delta_t: 24.000 }, // Year 1685.50
    DeltaTValue{jd: 2336859.50, delta_t: 24.000 }, // Year 1686.00
    DeltaTValue{jd: 2337042.00, delta_t: 24.000 }, // Year 1686.50
    DeltaTValue{jd: 2337224.50, delta_t: 23.000 }, // Year 1687.00
    DeltaTValue{jd: 2337407.00, delta_t: 23.000 }, // Year 1687.50
    DeltaTValue{jd: 2337589.50, delta_t: 23.000 }, // Year 1688.00
    DeltaTValue{jd: 2337772.50, delta_t: 23.000 }, // Year 1688.50
    DeltaTValue{jd: 2337955.50, delta_t: 22.000 }, // Year 1689.00
    DeltaTValue{jd: 2338138.00, delta_t: 22.000 }, // Year 1689.50
    DeltaTValue{jd: 2338320.50, delta_t: 22.000 }, // Year 1690.00
    DeltaTValue{jd: 2338503.00, delta_t: 22.000 }, // Year 1690.50
    DeltaTValue{jd: 2338685.50, delta_t: 22.000 }, // Year 1691.00
    DeltaTValue{jd: 2338868.00, delta_t: 21.000 }, // Year 1691.50
    DeltaTValue{jd: 2339050.50, delta_t: 21.000 }, // Year 1692.00
    DeltaTValue{jd: 2339233.50, delta_t: 21.000 }, // Year 1692.50
    DeltaTValue{jd: 2339416.50, delta_t: 21.000 }, // Year 1693.00
    DeltaTValue{jd: 2339599.00, delta_t: 21.000 }, // Year 1693.50
    DeltaTValue{jd: 2339781.50, delta_t: 21.000 }, // Year 1694.00
    DeltaTValue{jd: 2339964.00, delta_t: 21.000 }, // Year 1694.50
    DeltaTValue{jd: 2340146.50, delta_t: 21.000 }, // Year 1695.00
    DeltaTValue{jd: 2340329.00, delta_t: 20.000 }, // Year 1695.50
    DeltaTValue{jd: 2340511.50, delta_t: 20.000 }, // Year 1696.00
    DeltaTValue{jd: 2340694.50, delta_t: 20.000 }, // Year 1696.50
    DeltaTValue{jd: 2340877.50, delta_t: 20.000 }, // Year 1697.00
    DeltaTValue{jd: 2341060.00, delta_t: 20.000 }, // Year 1697.50
    DeltaTValue{jd: 2341242.50, delta_t: 20.000 }, // Year 1698.00
    DeltaTValue{jd: 2341425.00, delta_t: 20.000 }, // Year 1698.50
    DeltaTValue{jd: 2341607.50, delta_t: 20.000 }, // Year 1699.00
    DeltaTValue{jd: 2341790.00, delta_t: 20.000 }, // Year 1699.50
    DeltaTValue{jd: 2341972.50, delta_t: 21.000 }, // Year 1700.00
    DeltaTValue{jd: 2342155.00, delta_t: 21.000 }, // Year 1700.50
    DeltaTValue{jd: 2342337.50, delta_t: 21.000 }, // Year 1701.00
    DeltaTValue{jd: 2342520.00, delta_t: 20.000 }, // Year 1701.50
    DeltaTValue{jd: 2342702.50, delta_t: 20.000 }, // Year 1702.00
    DeltaTValue{jd: 2342885.00, delta_t: 20.000 }, // Year 1702.50
    DeltaTValue{jd: 2343067.50, delta_t: 20.000 }, // Year 1703.00
    DeltaTValue{jd: 2343250.00, delta_t: 20.000 }, // Year 1703.50
    DeltaTValue{jd: 2343432.50, delta_t: 19.000 }, // Year 1704.00
    DeltaTValue{jd: 2343615.50, delta_t: 19.000 }, // Year 1704.50
    DeltaTValue{jd: 2343798.50, delta_t: 19.000 }, // Year 1705.00
    DeltaTValue{jd: 2343981.00, delta_t: 19.000 }, // Year 1705.50
    DeltaTValue{jd: 2344163.50, delta_t: 19.000 }, // Year 1706.00
    DeltaTValue{jd: 2344346.00, delta_t: 20.000 }, // Year 1706.50
    DeltaTValue{jd: 2344528.50, delta_t: 20.000 }, // Year 1707.00
    DeltaTValue{jd: 2344711.00, delta_t: 20.000 }, // Year 1707.50
    DeltaTValue{jd: 2344893.50, delta_t: 20.000 }, // Year 1708.00
    DeltaTValue{jd: 2345076.50, delta_t: 19.000 }, // Year 1708.50
    DeltaTValue{jd: 2345259.50, delta_t: 20.000 }, // Year 1709.00
    DeltaTValue{jd: 2345442.00, delta_t: 20.000 }, // Year 1709.50
    DeltaTValue{jd: 2345624.50, delta_t: 20.000 }, // Year 1710.00
    DeltaTValue{jd: 2345807.00, delta_t: 20.000 }, // Year 1710.50
    DeltaTValue{jd: 2345989.50, delta_t: 20.000 }, // Year 1711.00
    DeltaTValue{jd: 2346172.00, delta_t: 20.000 }, // Year 1711.50
    DeltaTValue{jd: 2346354.50, delta_t: 21.000 }, // Year 1712.00
    DeltaTValue{jd: 2346537.50, delta_t: 21.000 }, // Year 1712.50
    DeltaTValue{jd: 2346720.50, delta_t: 21.000 }, // Year 1713.00
    DeltaTValue{jd: 2346903.00, delta_t: 21.000 }, // Year 1713.50
    DeltaTValue{jd: 2347085.50, delta_t: 21.000 }, // Year 1714.00
    DeltaTValue{jd: 2347268.00, delta_t: 21.000 }, // Year 1714.50
    DeltaTValue{jd: 2347450.50, delta_t: 21.000 }, // Year 1715.00
    DeltaTValue{jd: 2347633.00, delta_t: 21.000 }, // Year 1715.50
    DeltaTValue{jd: 2347815.50, delta_t: 21.000 }, // Year 1716.00
    DeltaTValue{jd: 2347998.50, delta_t: 21.000 }, // Year 1716.50
    DeltaTValue{jd: 2348181.50, delta_t: 21.000 }, // Year 1717.00
    DeltaTValue{jd: 2348364.00, delta_t: 21.000 }, // Year 1717.50
    DeltaTValue{jd: 2348546.50, delta_t: 21.000 }, // Year 1718.00
    DeltaTValue{jd: 2348729.00, delta_t: 21.000 }, // Year 1718.50
    DeltaTValue{jd: 2348911.50, delta_t: 21.000 }, // Year 1719.00
    DeltaTValue{jd: 2349094.00, delta_t: 21.000 }, // Year 1719.50
    DeltaTValue{jd: 2349276.50, delta_t: 21.100 }, // Year 1720.00
    DeltaTValue{jd: 2349459.50, delta_t: 21.000 }, // Year 1720.50
    DeltaTValue{jd: 2349642.50, delta_t: 21.000 }, // Year 1721.00
    DeltaTValue{jd: 2349825.00, delta_t: 21.000 }, // Year 1721.50
    DeltaTValue{jd: 2350007.50, delta_t: 20.900 }, // Year 1722.00
    DeltaTValue{jd: 2350190.00, delta_t: 20.800 }, // Year 1722.50
    DeltaTValue{jd: 2350372.50, delta_t: 20.700 }, // Year 1723.00
    DeltaTValue{jd: 2350555.00, delta_t: 20.600 }, // Year 1723.50
    DeltaTValue{jd: 2350737.50, delta_t: 20.400 }, // Year 1724.00
    DeltaTValue{jd: 2350920.50, delta_t: 20.200 }, // Year 1724.50
    DeltaTValue{jd: 2351103.50, delta_t: 20.000 }, // Year 1725.00
    DeltaTValue{jd: 2351286.00, delta_t: 19.700 }, // Year 1725.50
    DeltaTValue{jd: 2351468.50, delta_t: 19.400 }, // Year 1726.00
    DeltaTValue{jd: 2351651.00, delta_t: 19.100 }, // Year 1726.50
    DeltaTValue{jd: 2351833.50, delta_t: 18.700 }, // Year 1727.00
    DeltaTValue{jd: 2352016.00, delta_t: 18.300 }, // Year 1727.50
    DeltaTValue{jd: 2352198.50, delta_t: 17.800 }, // Year 1728.00
    DeltaTValue{jd: 2352381.50, delta_t: 17.400 }, // Year 1728.50
    DeltaTValue{jd: 2352564.50, delta_t: 17.000 }, // Year 1729.00
    DeltaTValue{jd: 2352747.00, delta_t: 16.800 }, // Year 1729.50
    DeltaTValue{jd: 2352929.50, delta_t: 16.600 }, // Year 1730.00
    DeltaTValue{jd: 2353112.00, delta_t: 16.400 }, // Year 1730.50
    DeltaTValue{jd: 2353294.50, delta_t: 16.100 }, // Year 1731.00
    DeltaTValue{jd: 2353477.00, delta_t: 15.900 }, // Year 1731.50
    DeltaTValue{jd: 2353659.50, delta_t: 15.700 }, // Year 1732.00
    DeltaTValue{jd: 2353842.50, delta_t: 15.500 }, // Year 1732.50
    DeltaTValue{jd: 2354025.50, delta_t: 15.300 }, // Year 1733.00
    DeltaTValue{jd: 2354208.00, delta_t: 15.000 }, // Year 1733.50
    DeltaTValue{jd: 2354390.50, delta_t: 14.700 }, // Year 1734.00
    DeltaTValue{jd: 2354573.00, delta_t: 14.500 }, // Year 1734.50
    DeltaTValue{jd: 2354755.50, delta_t: 14.300 }, // Year 1735.00
    DeltaTValue{jd: 2354938.00, delta_t: 14.200 }, // Year 1735.50
    DeltaTValue{jd: 2355120.50, delta_t: 14.100 }, // Year 1736.00
    DeltaTValue{jd: 2355303.50, delta_t: 14.100 }, // Year 1736.50
    DeltaTValue{jd: 2355486.50, delta_t: 14.100 }, // Year 1737.00
    DeltaTValue{jd: 2355669.00, delta_t: 13.900 }, // Year 1737.50
    DeltaTValue{jd: 2355851.50, delta_t: 13.700 }, // Year 1738.00
    DeltaTValue{jd: 2356034.00, delta_t: 13.600 }, // Year 1738.50
    DeltaTValue{jd: 2356216.50, delta_t: 13.500 }, // Year 1739.00
    DeltaTValue{jd: 2356399.00, delta_t: 13.500 }, // Year 1739.50
    DeltaTValue{jd: 2356581.50, delta_t: 13.500 }, // Year 1740.00
    DeltaTValue{jd: 2356764.50, delta_t: 13.500 }, // Year 1740.50
    DeltaTValue{jd: 2356947.50, delta_t: 13.400 }, // Year 1741.00
    DeltaTValue{jd: 2357130.00, delta_t: 13.400 }, // Year 1741.50
    DeltaTValue{jd: 2357312.50, delta_t: 13.400 }, // Year 1742.00
    DeltaTValue{jd: 2357495.00, delta_t: 13.400 }, // Year 1742.50
    DeltaTValue{jd: 2357677.50, delta_t: 13.300 }, // Year 1743.00
    DeltaTValue{jd: 2357860.00, delta_t: 13.300 }, // Year 1743.50
    DeltaTValue{jd: 2358042.50, delta_t: 13.200 }, // Year 1744.00
    DeltaTValue{jd: 2358225.50, delta_t: 13.200 }, // Year 1744.50
    DeltaTValue{jd: 2358408.50, delta_t: 13.200 }, // Year 1745.00
    DeltaTValue{jd: 2358591.00, delta_t: 13.100 }, // Year 1745.50
    DeltaTValue{jd: 2358773.50, delta_t: 13.100 }, // Year 1746.00
    DeltaTValue{jd: 2358956.00, delta_t: 13.100 }, // Year 1746.50
    DeltaTValue{jd: 2359138.50, delta_t: 13.000 }, // Year 1747.00
    DeltaTValue{jd: 2359321.00, delta_t: 13.200 }, // Year 1747.50
    DeltaTValue{jd: 2359503.50, delta_t: 13.300 }, // Year 1748.00
    DeltaTValue{jd: 2359686.50, delta_t: 13.400 }, // Year 1748.50
    DeltaTValue{jd: 2359869.50, delta_t: 13.500 }, // Year 1749.00
    DeltaTValue{jd: 2360052.00, delta_t: 13.600 }, // Year 1749.50
    DeltaTValue{jd: 2360234.50, delta_t: 13.700 }, // Year 1750.00
    DeltaTValue{jd: 2360417.00, delta_t: 13.800 }, // Year 1750.50
    DeltaTValue{jd: 2360599.50, delta_t: 13.900 }, // Year 1751.00
    DeltaTValue{jd: 2360782.00, delta_t: 14.000 }, // Year 1751.50
    DeltaTValue{jd: 2360964.50, delta_t: 14.000 }, // Year 1752.00
    DeltaTValue{jd: 2361147.50, delta_t: 14.100 }, // Year 1752.50
    DeltaTValue{jd: 2361330.50, delta_t: 14.100 }, // Year 1753.00
    DeltaTValue{jd: 2361513.00, delta_t: 14.100 }, // Year 1753.50
    DeltaTValue{jd: 2361695.50, delta_t: 14.100 }, // Year 1754.00
    DeltaTValue{jd: 2361878.00, delta_t: 14.200 }, // Year 1754.50
    DeltaTValue{jd: 2362060.50, delta_t: 14.300 }, // Year 1755.00
    DeltaTValue{jd: 2362243.00, delta_t: 14.400 }, // Year 1755.50
    DeltaTValue{jd: 2362425.50, delta_t: 14.400 }, // Year 1756.00
    DeltaTValue{jd: 2362608.50, delta_t: 14.500 }, // Year 1756.50
    DeltaTValue{jd: 2362791.50, delta_t: 14.600 }, // Year 1757.00
    DeltaTValue{jd: 2362974.00, delta_t: 14.600 }, // Year 1757.50
    DeltaTValue{jd: 2363156.50, delta_t: 14.700 }, // Year 1758.00
    DeltaTValue{jd: 2363339.00, delta_t: 14.700 }, // Year 1758.50
    DeltaTValue{jd: 2363521.50, delta_t: 14.700 }, // Year 1759.00
    DeltaTValue{jd: 2363704.00, delta_t: 14.800 }, // Year 1759.50
    DeltaTValue{jd: 2363886.50, delta_t: 14.800 }, // Year 1760.00
    DeltaTValue{jd: 2364069.50, delta_t: 14.900 }, // Year 1760.50
    DeltaTValue{jd: 2364252.50, delta_t: 14.900 }, // Year 1761.00
    DeltaTValue{jd: 2364435.00, delta_t: 15.000 }, // Year 1761.50
    DeltaTValue{jd: 2364617.50, delta_t: 15.000 }, // Year 1762.00
    DeltaTValue{jd: 2364800.00, delta_t: 15.100 }, // Year 1762.50
    DeltaTValue{jd: 2364982.50, delta_t: 15.200 }, // Year 1763.00
    DeltaTValue{jd: 2365165.00, delta_t: 15.300 }, // Year 1763.50
    DeltaTValue{jd: 2365347.50, delta_t: 15.400 }, // Year 1764.00
    DeltaTValue{jd: 2365530.50, delta_t: 15.500 }, // Year 1764.50
    DeltaTValue{jd: 2365713.50, delta_t: 15.600 }, // Year 1765.00
    DeltaTValue{jd: 2365896.00, delta_t: 15.600 }, // Year 1765.50
    DeltaTValue{jd: 2366078.50, delta_t: 15.600 }, // Year 1766.00
    DeltaTValue{jd: 2366261.00, delta_t: 15.800 }, // Year 1766.50
    DeltaTValue{jd: 2366443.50, delta_t: 15.900 }, // Year 1767.00
    DeltaTValue{jd: 2366626.00, delta_t: 15.900 }, // Year 1767.50
    DeltaTValue{jd: 2366808.50, delta_t: 15.900 }, // Year 1768.00
    DeltaTValue{jd: 2366991.50, delta_t: 15.800 }, // Year 1768.50
    DeltaTValue{jd: 2367174.50, delta_t: 15.700 }, // Year 1769.00
    DeltaTValue{jd: 2367357.00, delta_t: 15.800 }, // Year 1769.50
    DeltaTValue{jd: 2367539.50, delta_t: 15.700 }, // Year 1770.00
    DeltaTValue{jd: 2367722.00, delta_t: 15.700 }, // Year 1770.50
    DeltaTValue{jd: 2367904.50, delta_t: 15.700 }, // Year 1771.00
    DeltaTValue{jd: 2368087.00, delta_t: 15.800 }, // Year 1771.50
    DeltaTValue{jd: 2368269.50, delta_t: 15.900 }, // Year 1772.00
    DeltaTValue{jd: 2368452.50, delta_t: 16.100 }, // Year 1772.50
    DeltaTValue{jd: 2368635.50, delta_t: 16.100 }, // Year 1773.00
    DeltaTValue{jd: 2368818.00, delta_t: 16.000 }, // Year 1773.50
    DeltaTValue{jd: 2369000.50, delta_t: 15.900 }, // Year 1774.00
    DeltaTValue{jd: 2369183.00, delta_t: 15.900 }, // Year 1774.50
    DeltaTValue{jd: 2369365.50, delta_t: 15.700 }, // Year 1775.00
    DeltaTValue{jd: 2369548.00, delta_t: 15.400 }, // Year 1775.50
    DeltaTValue{jd: 2369730.50, delta_t: 15.300 }, // Year 1776.00
    DeltaTValue{jd: 2369913.50, delta_t: 15.400 }, // Year 1776.50
    DeltaTValue{jd: 2370096.50, delta_t: 15.500 }, // Year 1777.00
    DeltaTValue{jd: 2370279.00, delta_t: 15.600 }, // Year 1777.50
    DeltaTValue{jd: 2370461.50, delta_t: 15.600 }, // Year 1778.00
    DeltaTValue{jd: 2370644.00, delta_t: 15.600 }, // Year 1778.50
    DeltaTValue{jd: 2370826.50, delta_t: 15.600 }, // Year 1779.00
    DeltaTValue{jd: 2371009.00, delta_t: 15.600 }, // Year 1779.50
    DeltaTValue{jd: 2371191.50, delta_t: 15.600 }, // Year 1780.00
    DeltaTValue{jd: 2371374.50, delta_t: 15.600 }, // Year 1780.50
    DeltaTValue{jd: 2371557.50, delta_t: 15.500 }, // Year 1781.00
    DeltaTValue{jd: 2371740.00, delta_t: 15.500 }, // Year 1781.50
    DeltaTValue{jd: 2371922.50, delta_t: 15.400 }, // Year 1782.00
    DeltaTValue{jd: 2372105.00, delta_t: 15.300 }, // Year 1782.50
    DeltaTValue{jd: 2372287.50, delta_t: 15.200 }, // Year 1783.00
    DeltaTValue{jd: 2372470.00, delta_t: 15.100 }, // Year 1783.50
    DeltaTValue{jd: 2372652.50, delta_t: 14.900 }, // Year 1784.00
    DeltaTValue{jd: 2372835.50, delta_t: 14.800 }, // Year 1784.50
    DeltaTValue{jd: 2373018.50, delta_t: 14.600 }, // Year 1785.00
    DeltaTValue{jd: 2373201.00, delta_t: 14.400 }, // Year 1785.50
    DeltaTValue{jd: 2373383.50, delta_t: 14.300 }, // Year 1786.00
    DeltaTValue{jd: 2373566.00, delta_t: 14.200 }, // Year 1786.50
    DeltaTValue{jd: 2373748.50, delta_t: 14.100 }, // Year 1787.00
    DeltaTValue{jd: 2373931.00, delta_t: 14.200 }, // Year 1787.50
    DeltaTValue{jd: 2374113.50, delta_t: 14.200 }, // Year 1788.00
    DeltaTValue{jd: 2374296.50, delta_t: 13.900 }, // Year 1788.50
    DeltaTValue{jd: 2374479.50, delta_t: 13.700 }, // Year 1789.00
    DeltaTValue{jd: 2374662.00, delta_t: 13.500 }, // Year 1789.50
    DeltaTValue{jd: 2374844.50, delta_t: 13.300 }, // Year 1790.00
    DeltaTValue{jd: 2375027.00, delta_t: 13.100 }, // Year 1790.50
    DeltaTValue{jd: 2375209.50, delta_t: 13.000 }, // Year 1791.00
    DeltaTValue{jd: 2375392.00, delta_t: 13.200 }, // Year 1791.50
    DeltaTValue{jd: 2375574.50, delta_t: 13.200 }, // Year 1792.00
    DeltaTValue{jd: 2375757.50, delta_t: 13.100 }, // Year 1792.50
    DeltaTValue{jd: 2375940.50, delta_t: 13.100 }, // Year 1793.00
    DeltaTValue{jd: 2376123.00, delta_t: 13.200 }, // Year 1793.50
    DeltaTValue{jd: 2376305.50, delta_t: 13.300 }, // Year 1794.00
    DeltaTValue{jd: 2376488.00, delta_t: 13.500 }, // Year 1794.50
    DeltaTValue{jd: 2376670.50, delta_t: 13.500 }, // Year 1795.00
    DeltaTValue{jd: 2376853.00, delta_t: 13.400 }, // Year 1795.50
    DeltaTValue{jd: 2377035.50, delta_t: 13.200 }, // Year 1796.00
    DeltaTValue{jd: 2377218.50, delta_t: 13.200 }, // Year 1796.50
    DeltaTValue{jd: 2377401.50, delta_t: 13.100 }, // Year 1797.00
    DeltaTValue{jd: 2377584.00, delta_t: 13.100 }, // Year 1797.50
    DeltaTValue{jd: 2377766.50, delta_t: 13.000 }, // Year 1798.00
    DeltaTValue{jd: 2377949.00, delta_t: 12.800 }, // Year 1798.50
    DeltaTValue{jd: 2378131.50, delta_t: 12.600 }, // Year 1799.00
    DeltaTValue{jd: 2378314.00, delta_t: 12.700 }, // Year 1799.50
    DeltaTValue{jd: 2378496.50, delta_t: 12.600 }, // Year 1800.00
    DeltaTValue{jd: 2378679.00, delta_t: 12.300 }, // Year 1800.50
    DeltaTValue{jd: 2378861.50, delta_t: 12.000 }, // Year 1801.00
    DeltaTValue{jd: 2379044.00, delta_t: 11.900 }, // Year 1801.50
    DeltaTValue{jd: 2379226.50, delta_t: 11.800 }, // Year 1802.00
    DeltaTValue{jd: 2379409.00, delta_t: 11.600 }, // Year 1802.50
    DeltaTValue{jd: 2379591.50, delta_t: 11.400 }, // Year 1803.00
    DeltaTValue{jd: 2379774.00, delta_t: 11.200 }, // Year 1803.50
    DeltaTValue{jd: 2379956.50, delta_t: 11.100 }, // Year 1804.00
    DeltaTValue{jd: 2380139.50, delta_t: 11.100 }, // Year 1804.50
    DeltaTValue{jd: 2380322.50, delta_t: 11.100 }, // Year 1805.00
    DeltaTValue{jd: 2380505.00, delta_t: 11.100 }, // Year 1805.50
    DeltaTValue{jd: 2380687.50, delta_t: 11.100 }, // Year 1806.00
    DeltaTValue{jd: 2380870.00, delta_t: 11.200 }, // Year 1806.50
    DeltaTValue{jd: 2381052.50, delta_t: 11.100 }, // Year 1807.00
    DeltaTValue{jd: 2381235.00, delta_t: 11.100 }, // Year 1807.50
    DeltaTValue{jd: 2381417.50, delta_t: 11.200 }, // Year 1808.00
    DeltaTValue{jd: 2381600.50, delta_t: 11.400 }, // Year 1808.50
    DeltaTValue{jd: 2381783.50, delta_t: 11.500 }, // Year 1809.00
    DeltaTValue{jd: 2381966.00, delta_t: 11.300 }, // Year 1809.50
    DeltaTValue{jd: 2382148.50, delta_t: 11.200 }, // Year 1810.00
    DeltaTValue{jd: 2382331.00, delta_t: 11.400 }, // Year 1810.50
    DeltaTValue{jd: 2382513.50, delta_t: 11.700 }, // Year 1811.00
    DeltaTValue{jd: 2382696.00, delta_t: 11.900 }, // Year 1811.50
    DeltaTValue{jd: 2382878.50, delta_t: 11.900 }, // Year 1812.00
    DeltaTValue{jd: 2383061.50, delta_t: 11.900 }, // Year 1812.50
    DeltaTValue{jd: 2383244.50, delta_t: 11.800 }, // Year 1813.00
    DeltaTValue{jd: 2383427.00, delta_t: 11.700 }, // Year 1813.50
    DeltaTValue{jd: 2383609.50, delta_t: 11.800 }, // Year 1814.00
    DeltaTValue{jd: 2383792.00, delta_t: 11.800 }, // Year 1814.50
    DeltaTValue{jd: 2383974.50, delta_t: 11.800 }, // Year 1815.00
    DeltaTValue{jd: 2384157.00, delta_t: 11.700 }, // Year 1815.50
    DeltaTValue{jd: 2384339.50, delta_t: 11.600 }, // Year 1816.00
    DeltaTValue{jd: 2384522.50, delta_t: 11.600 }, // Year 1816.50
    DeltaTValue{jd: 2384705.50, delta_t: 11.500 }, // Year 1817.00
    DeltaTValue{jd: 2384888.00, delta_t: 11.500 }, // Year 1817.50
    DeltaTValue{jd: 2385070.50, delta_t: 11.400 }, // Year 1818.00
    DeltaTValue{jd: 2385253.00, delta_t: 11.400 }, // Year 1818.50
    DeltaTValue{jd: 2385435.50, delta_t: 11.300 }, // Year 1819.00
    DeltaTValue{jd: 2385618.00, delta_t: 11.300 }, // Year 1819.50
    DeltaTValue{jd: 2385800.50, delta_t: 11.130 }, // Year 1820.00
    DeltaTValue{jd: 2385983.50, delta_t: 11.160 }, // Year 1820.50
    DeltaTValue{jd: 2386166.50, delta_t: 10.940 }, // Year 1821.00
    DeltaTValue{jd: 2386349.00, delta_t: 10.720 }, // Year 1821.50
    DeltaTValue{jd: 2386531.50, delta_t: 10.290 }, // Year 1822.00
    DeltaTValue{jd: 2386714.00, delta_t: 10.040 }, // Year 1822.50
    DeltaTValue{jd: 2386896.50, delta_t: 9.940  },// Year 1823.00
    DeltaTValue{jd: 2387079.00, delta_t: 9.910  },// Year 1823.50
    DeltaTValue{jd: 2387261.50, delta_t: 9.880  },// Year 1824.00
    DeltaTValue{jd: 2387444.50, delta_t: 9.860  },// Year 1824.50
    DeltaTValue{jd: 2387627.50, delta_t: 9.720  },// Year 1825.00
    DeltaTValue{jd: 2387810.00, delta_t: 9.670  },// Year 1825.50
    DeltaTValue{jd: 2387992.50, delta_t: 9.660  },// Year 1826.00
    DeltaTValue{jd: 2388175.00, delta_t: 9.640  },// Year 1826.50
    DeltaTValue{jd: 2388357.50, delta_t: 9.510  },// Year 1827.00
    DeltaTValue{jd: 2388540.00, delta_t: 9.400  },// Year 1827.50
    DeltaTValue{jd: 2388722.50, delta_t: 9.210  },// Year 1828.00
    DeltaTValue{jd: 2388905.50, delta_t: 9.000  },// Year 1828.50
    DeltaTValue{jd: 2389088.50, delta_t: 8.600  },// Year 1829.00
    DeltaTValue{jd: 2389271.00, delta_t: 8.290  },// Year 1829.50
    DeltaTValue{jd: 2389453.50, delta_t: 7.950  },// Year 1830.00
    DeltaTValue{jd: 2389636.00, delta_t: 7.730  },// Year 1830.50
    DeltaTValue{jd: 2389818.50, delta_t: 7.590  },// Year 1831.00
    DeltaTValue{jd: 2390001.00, delta_t: 7.490  },// Year 1831.50
    DeltaTValue{jd: 2390183.50, delta_t: 7.360  },// Year 1832.00
    DeltaTValue{jd: 2390366.50, delta_t: 7.260  },// Year 1832.50
    DeltaTValue{jd: 2390549.50, delta_t: 7.100  },// Year 1833.00
    DeltaTValue{jd: 2390732.00, delta_t: 7.000  },// Year 1833.50
    DeltaTValue{jd: 2390914.50, delta_t: 6.890  },// Year 1834.00
    DeltaTValue{jd: 2391097.00, delta_t: 6.820  },// Year 1834.50
    DeltaTValue{jd: 2391279.50, delta_t: 6.730  },// Year 1835.00
    DeltaTValue{jd: 2391462.00, delta_t: 6.640  },// Year 1835.50
    DeltaTValue{jd: 2391644.50, delta_t: 6.390  },// Year 1836.00
    DeltaTValue{jd: 2391827.50, delta_t: 6.280  },// Year 1836.50
    DeltaTValue{jd: 2392010.50, delta_t: 6.250  },// Year 1837.00
    DeltaTValue{jd: 2392193.00, delta_t: 6.270  },// Year 1837.50
    DeltaTValue{jd: 2392375.50, delta_t: 6.250  },// Year 1838.00
    DeltaTValue{jd: 2392558.00, delta_t: 6.270  },// Year 1838.50
    DeltaTValue{jd: 2392740.50, delta_t: 6.220  },// Year 1839.00
    DeltaTValue{jd: 2392923.00, delta_t: 6.240  },// Year 1839.50
    DeltaTValue{jd: 2393105.50, delta_t: 6.220  },// Year 1840.00
    DeltaTValue{jd: 2393288.50, delta_t: 6.270  },// Year 1840.50
    DeltaTValue{jd: 2393471.50, delta_t: 6.300  },// Year 1841.00
    DeltaTValue{jd: 2393654.00, delta_t: 6.360  },// Year 1841.50
    DeltaTValue{jd: 2393836.50, delta_t: 6.350  },// Year 1842.00
    DeltaTValue{jd: 2394019.00, delta_t: 6.370  },// Year 1842.50
    DeltaTValue{jd: 2394201.50, delta_t: 6.320  },// Year 1843.00
    DeltaTValue{jd: 2394384.00, delta_t: 6.330  },// Year 1843.50
    DeltaTValue{jd: 2394566.50, delta_t: 6.330  },// Year 1844.00
    DeltaTValue{jd: 2394749.50, delta_t: 6.370  },// Year 1844.50
    DeltaTValue{jd: 2394932.50, delta_t: 6.370  },// Year 1845.00
    DeltaTValue{jd: 2395115.00, delta_t: 6.410  },// Year 1845.50
    DeltaTValue{jd: 2395297.50, delta_t: 6.400  },// Year 1846.00
    DeltaTValue{jd: 2395480.00, delta_t: 6.440  },// Year 1846.50
    DeltaTValue{jd: 2395662.50, delta_t: 6.460  },// Year 1847.00
    DeltaTValue{jd: 2395845.00, delta_t: 6.510  },// Year 1847.50
    DeltaTValue{jd: 2396027.50, delta_t: 6.480  },// Year 1848.00
    DeltaTValue{jd: 2396210.50, delta_t: 6.510  },// Year 1848.50
    DeltaTValue{jd: 2396393.50, delta_t: 6.530  },// Year 1849.00
    DeltaTValue{jd: 2396576.00, delta_t: 6.580  },// Year 1849.50
    DeltaTValue{jd: 2396758.50, delta_t: 6.550  },// Year 1850.00
    DeltaTValue{jd: 2396941.00, delta_t: 6.610  },// Year 1850.50
    DeltaTValue{jd: 2397123.50, delta_t: 6.690  },// Year 1851.00
    DeltaTValue{jd: 2397306.00, delta_t: 6.800  },// Year 1851.50
    DeltaTValue{jd: 2397488.50, delta_t: 6.840  },// Year 1852.00
    DeltaTValue{jd: 2397671.50, delta_t: 6.940  },// Year 1852.50
    DeltaTValue{jd: 2397854.50, delta_t: 7.030  },// Year 1853.00
    DeltaTValue{jd: 2398037.00, delta_t: 7.130  },// Year 1853.50
    DeltaTValue{jd: 2398219.50, delta_t: 7.150  },// Year 1854.00
    DeltaTValue{jd: 2398402.00, delta_t: 7.220  },// Year 1854.50
    DeltaTValue{jd: 2398584.50, delta_t: 7.260  },// Year 1855.00
    DeltaTValue{jd: 2398767.00, delta_t: 7.300  },// Year 1855.50
    DeltaTValue{jd: 2398949.50, delta_t: 7.230  },// Year 1856.00
    DeltaTValue{jd: 2399132.50, delta_t: 7.220  },// Year 1856.50
    DeltaTValue{jd: 2399315.50, delta_t: 7.210  },// Year 1857.00
    DeltaTValue{jd: 2399498.00, delta_t: 7.200  },// Year 1857.50
    DeltaTValue{jd: 2399680.50, delta_t: 6.990  },// Year 1858.00
    DeltaTValue{jd: 2399863.00, delta_t: 6.980  },// Year 1858.50
    DeltaTValue{jd: 2400045.50, delta_t: 7.190  },// Year 1859.00
    DeltaTValue{jd: 2400228.00, delta_t: 7.360  },// Year 1859.50
    DeltaTValue{jd: 2400410.50, delta_t: 7.350  },// Year 1860.00
    DeltaTValue{jd: 2400593.50, delta_t: 7.390  },// Year 1860.50
    DeltaTValue{jd: 2400776.50, delta_t: 7.410  },// Year 1861.00
    DeltaTValue{jd: 2400959.00, delta_t: 7.450  },// Year 1861.50
    DeltaTValue{jd: 2401141.50, delta_t: 7.360  },// Year 1862.00
    DeltaTValue{jd: 2401324.00, delta_t: 7.180  },// Year 1862.50
    DeltaTValue{jd: 2401506.50, delta_t: 6.950  },// Year 1863.00
    DeltaTValue{jd: 2401689.00, delta_t: 6.720  },// Year 1863.50
    DeltaTValue{jd: 2401871.50, delta_t: 6.450  },// Year 1864.00
    DeltaTValue{jd: 2402054.50, delta_t: 6.240  },// Year 1864.50
    DeltaTValue{jd: 2402237.50, delta_t: 5.920  },// Year 1865.00
    DeltaTValue{jd: 2402420.00, delta_t: 5.590  },// Year 1865.50
    DeltaTValue{jd: 2402602.50, delta_t: 5.150  },// Year 1866.00
    DeltaTValue{jd: 2402785.00, delta_t: 4.670  },// Year 1866.50
    DeltaTValue{jd: 2402967.50, delta_t: 4.110  },// Year 1867.00
    DeltaTValue{jd: 2403150.00, delta_t: 3.520  },// Year 1867.50
    DeltaTValue{jd: 2403332.50, delta_t: 2.940  },// Year 1868.00
    DeltaTValue{jd: 2403515.50, delta_t: 2.470  },// Year 1868.50
    DeltaTValue{jd: 2403698.50, delta_t: 1.970  },// Year 1869.00
    DeltaTValue{jd: 2403881.00, delta_t: 1.520  },// Year 1869.50
    DeltaTValue{jd: 2404063.50, delta_t: 1.040  },// Year 1870.00
    DeltaTValue{jd: 2404246.00, delta_t: 0.600  },// Year 1870.50
    DeltaTValue{jd: 2404428.50, delta_t: 0.110  },// Year 1871.00
    DeltaTValue{jd: 2404611.00, delta_t: -0.340 }, // Year 1871.50
    DeltaTValue{jd: 2404793.50, delta_t: -0.820 }, // Year 1872.00
    DeltaTValue{jd: 2404976.50, delta_t: -1.250 }, // Year 1872.50
    DeltaTValue{jd: 2405159.50, delta_t: -1.700 }, // Year 1873.00
    DeltaTValue{jd: 2405342.00, delta_t: -2.080 }, // Year 1873.50
    DeltaTValue{jd: 2405524.50, delta_t: -2.480 }, // Year 1874.00
    DeltaTValue{jd: 2405707.00, delta_t: -2.820 }, // Year 1874.50
    DeltaTValue{jd: 2405889.50, delta_t: -3.190 }, // Year 1875.00
    DeltaTValue{jd: 2406072.00, delta_t: -3.500 }, // Year 1875.50
    DeltaTValue{jd: 2406254.50, delta_t: -3.840 }, // Year 1876.00
    DeltaTValue{jd: 2406437.50, delta_t: -4.140 }, // Year 1876.50
    DeltaTValue{jd: 2406620.50, delta_t: -4.430 }, // Year 1877.00
    DeltaTValue{jd: 2406803.00, delta_t: -4.590 }, // Year 1877.50
    DeltaTValue{jd: 2406985.50, delta_t: -4.790 }, // Year 1878.00
    DeltaTValue{jd: 2407168.00, delta_t: -4.920 }, // Year 1878.50
    DeltaTValue{jd: 2407350.50, delta_t: -5.090 }, // Year 1879.00
    DeltaTValue{jd: 2407533.00, delta_t: -5.240 }, // Year 1879.50
    DeltaTValue{jd: 2407715.50, delta_t: -5.360 }, // Year 1880.00
    DeltaTValue{jd: 2407898.50, delta_t: -5.340 }, // Year 1880.50
    DeltaTValue{jd: 2408081.50, delta_t: -5.370 }, // Year 1881.00
    DeltaTValue{jd: 2408264.00, delta_t: -5.320 }, // Year 1881.50
    DeltaTValue{jd: 2408446.50, delta_t: -5.340 }, // Year 1882.00
    DeltaTValue{jd: 2408629.00, delta_t: -5.330 }, // Year 1882.50
    DeltaTValue{jd: 2408811.50, delta_t: -5.400 }, // Year 1883.00
    DeltaTValue{jd: 2408994.00, delta_t: -5.470 }, // Year 1883.50
    DeltaTValue{jd: 2409176.50, delta_t: -5.580 }, // Year 1884.00
    DeltaTValue{jd: 2409359.50, delta_t: -5.660 }, // Year 1884.50
    DeltaTValue{jd: 2409542.50, delta_t: -5.740 }, // Year 1885.00
    DeltaTValue{jd: 2409725.00, delta_t: -5.680 }, // Year 1885.50
    DeltaTValue{jd: 2409907.50, delta_t: -5.690 }, // Year 1886.00
    DeltaTValue{jd: 2410090.00, delta_t: -5.650 }, // Year 1886.50
    DeltaTValue{jd: 2410272.50, delta_t: -5.670 }, // Year 1887.00
    DeltaTValue{jd: 2410455.00, delta_t: -5.680 }, // Year 1887.50
    DeltaTValue{jd: 2410637.50, delta_t: -5.730 }, // Year 1888.00
    DeltaTValue{jd: 2410820.50, delta_t: -5.720 }, // Year 1888.50
    DeltaTValue{jd: 2411003.50, delta_t: -5.780 }, // Year 1889.00
    DeltaTValue{jd: 2411186.00, delta_t: -5.790 }, // Year 1889.50
    DeltaTValue{jd: 2411368.50, delta_t: -5.860 }, // Year 1890.00
    DeltaTValue{jd: 2411551.00, delta_t: -5.890 }, // Year 1890.50
    DeltaTValue{jd: 2411733.50, delta_t: -6.010 }, // Year 1891.00
    DeltaTValue{jd: 2411916.00, delta_t: -6.130 }, // Year 1891.50
    DeltaTValue{jd: 2412098.50, delta_t: -6.280 }, // Year 1892.00
    DeltaTValue{jd: 2412281.50, delta_t: -6.410 }, // Year 1892.50
    DeltaTValue{jd: 2412464.50, delta_t: -6.530 }, // Year 1893.00
    DeltaTValue{jd: 2412647.00, delta_t: -6.490 }, // Year 1893.50
    DeltaTValue{jd: 2412829.50, delta_t: -6.500 }, // Year 1894.00
    DeltaTValue{jd: 2413012.00, delta_t: -6.450 }, // Year 1894.50
    DeltaTValue{jd: 2413194.50, delta_t: -6.410 }, // Year 1895.00
    DeltaTValue{jd: 2413377.00, delta_t: -6.260 }, // Year 1895.50
    DeltaTValue{jd: 2413559.50, delta_t: -6.110 }, // Year 1896.00
    DeltaTValue{jd: 2413742.50, delta_t: -5.900 }, // Year 1896.50
    DeltaTValue{jd: 2413925.50, delta_t: -5.630 }, // Year 1897.00
    DeltaTValue{jd: 2414108.00, delta_t: -5.130 }, // Year 1897.50
    DeltaTValue{jd: 2414290.50, delta_t: -4.680 }, // Year 1898.00
    DeltaTValue{jd: 2414473.00, delta_t: -4.190 }, // Year 1898.50
    DeltaTValue{jd: 2414655.50, delta_t: -3.720 }, // Year 1899.00
    DeltaTValue{jd: 2414838.00, delta_t: -3.210 }, // Year 1899.50
    DeltaTValue{jd: 2415020.50, delta_t: -2.700 }, // Year 1900.00
    DeltaTValue{jd: 2415203.00, delta_t: -2.090 }, // Year 1900.50
    DeltaTValue{jd: 2415385.50, delta_t: -1.480 }, // Year 1901.00
    DeltaTValue{jd: 2415568.00, delta_t: -0.750 }, // Year 1901.50
    DeltaTValue{jd: 2415750.50, delta_t: -0.080 }, // Year 1902.00
    DeltaTValue{jd: 2415933.00, delta_t: 0.620  },// Year 1902.50
    DeltaTValue{jd: 2416115.50, delta_t: 1.260  },// Year 1903.00
    DeltaTValue{jd: 2416298.00, delta_t: 1.950  },// Year 1903.50
    DeltaTValue{jd: 2416480.50, delta_t: 2.590  },// Year 1904.00
    DeltaTValue{jd: 2416663.50, delta_t: 3.280  },// Year 1904.50
    DeltaTValue{jd: 2416846.50, delta_t: 3.920  },// Year 1905.00
    DeltaTValue{jd: 2417029.00, delta_t: 4.610  },// Year 1905.50
    DeltaTValue{jd: 2417211.50, delta_t: 5.200  },// Year 1906.00
    DeltaTValue{jd: 2417394.00, delta_t: 5.730  },// Year 1906.50
    DeltaTValue{jd: 2417576.50, delta_t: 6.290  },// Year 1907.00
    DeltaTValue{jd: 2417759.00, delta_t: 7.000  },// Year 1907.50
    DeltaTValue{jd: 2417941.50, delta_t: 7.680  },// Year 1908.00
    DeltaTValue{jd: 2418124.50, delta_t: 8.450  },// Year 1908.50
    DeltaTValue{jd: 2418307.50, delta_t: 9.130  },// Year 1909.00
    DeltaTValue{jd: 2418490.00, delta_t: 9.780  },// Year 1909.50
    DeltaTValue{jd: 2418672.50, delta_t: 10.380 }, // Year 1910.00
    DeltaTValue{jd: 2418855.00, delta_t: 10.990 }, // Year 1910.50
    DeltaTValue{jd: 2419037.50, delta_t: 11.640 }, // Year 1911.00
    DeltaTValue{jd: 2419220.00, delta_t: 12.470 }, // Year 1911.50
    DeltaTValue{jd: 2419402.50, delta_t: 13.230 }, // Year 1912.00
    DeltaTValue{jd: 2419585.50, delta_t: 14.000 }, // Year 1912.50
    DeltaTValue{jd: 2419768.50, delta_t: 14.690 }, // Year 1913.00
    DeltaTValue{jd: 2419951.00, delta_t: 15.380 }, // Year 1913.50
    DeltaTValue{jd: 2420133.50, delta_t: 16.000 }, // Year 1914.00
    DeltaTValue{jd: 2420316.00, delta_t: 16.640 }, // Year 1914.50
    DeltaTValue{jd: 2420498.50, delta_t: 17.190 }, // Year 1915.00
    DeltaTValue{jd: 2420681.00, delta_t: 17.720 }, // Year 1915.50
    DeltaTValue{jd: 2420863.50, delta_t: 18.190 }, // Year 1916.00
    DeltaTValue{jd: 2421046.50, delta_t: 18.670 }, // Year 1916.50
    DeltaTValue{jd: 2421229.50, delta_t: 19.130 }, // Year 1917.00
    DeltaTValue{jd: 2421412.00, delta_t: 19.690 }, // Year 1917.50
    DeltaTValue{jd: 2421594.50, delta_t: 20.140 }, // Year 1918.00
    DeltaTValue{jd: 2421777.00, delta_t: 20.540 }, // Year 1918.50
    DeltaTValue{jd: 2421959.50, delta_t: 20.860 }, // Year 1919.00
    DeltaTValue{jd: 2422142.00, delta_t: 21.140 }, // Year 1919.50
    DeltaTValue{jd: 2422324.50, delta_t: 21.410 }, // Year 1920.00
    DeltaTValue{jd: 2422507.50, delta_t: 21.780 }, // Year 1920.50
    DeltaTValue{jd: 2422690.50, delta_t: 22.060 }, // Year 1921.00
    DeltaTValue{jd: 2422873.00, delta_t: 22.300 }, // Year 1921.50
    DeltaTValue{jd: 2423055.50, delta_t: 22.510 }, // Year 1922.00
    DeltaTValue{jd: 2423238.00, delta_t: 22.790 }, // Year 1922.50
    DeltaTValue{jd: 2423420.50, delta_t: 23.010 }, // Year 1923.00
    DeltaTValue{jd: 2423603.00, delta_t: 23.290 }, // Year 1923.50
    DeltaTValue{jd: 2423785.50, delta_t: 23.460 }, // Year 1924.00
    DeltaTValue{jd: 2423968.50, delta_t: 23.550 }, // Year 1924.50
    DeltaTValue{jd: 2424151.50, delta_t: 23.630 }, // Year 1925.00
    DeltaTValue{jd: 2424334.00, delta_t: 23.800 }, // Year 1925.50
    DeltaTValue{jd: 2424516.50, delta_t: 23.950 }, // Year 1926.00
    DeltaTValue{jd: 2424699.00, delta_t: 24.250 }, // Year 1926.50
    DeltaTValue{jd: 2424881.50, delta_t: 24.390 }, // Year 1927.00
    DeltaTValue{jd: 2425064.00, delta_t: 24.420 }, // Year 1927.50
    DeltaTValue{jd: 2425246.50, delta_t: 24.340 }, // Year 1928.00
    DeltaTValue{jd: 2425429.50, delta_t: 24.220 }, // Year 1928.50
    DeltaTValue{jd: 2425612.50, delta_t: 24.100 }, // Year 1929.00
    DeltaTValue{jd: 2425795.00, delta_t: 24.080 }, // Year 1929.50
    DeltaTValue{jd: 2425977.50, delta_t: 24.020 }, // Year 1930.00
    DeltaTValue{jd: 2426160.00, delta_t: 24.040 }, // Year 1930.50
    DeltaTValue{jd: 2426342.50, delta_t: 23.980 }, // Year 1931.00
    DeltaTValue{jd: 2426525.00, delta_t: 23.910 }, // Year 1931.50
    DeltaTValue{jd: 2426707.50, delta_t: 23.890 }, // Year 1932.00
    DeltaTValue{jd: 2426890.50, delta_t: 23.950 }, // Year 1932.50
    DeltaTValue{jd: 2427073.50, delta_t: 23.930 }, // Year 1933.00
    DeltaTValue{jd: 2427256.00, delta_t: 23.920 }, // Year 1933.50
    DeltaTValue{jd: 2427438.50, delta_t: 23.880 }, // Year 1934.00
    DeltaTValue{jd: 2427621.00, delta_t: 23.940 }, // Year 1934.50
    DeltaTValue{jd: 2427803.50, delta_t: 23.910 }, // Year 1935.00
    DeltaTValue{jd: 2427986.00, delta_t: 23.820 }, // Year 1935.50
    DeltaTValue{jd: 2428168.50, delta_t: 23.760 }, // Year 1936.00
    DeltaTValue{jd: 2428351.50, delta_t: 23.870 }, // Year 1936.50
    DeltaTValue{jd: 2428534.50, delta_t: 23.910 }, // Year 1937.00
    DeltaTValue{jd: 2428717.00, delta_t: 23.950 }, // Year 1937.50
    DeltaTValue{jd: 2428899.50, delta_t: 23.960 }, // Year 1938.00
    DeltaTValue{jd: 2429082.00, delta_t: 24.000 }, // Year 1938.50
    DeltaTValue{jd: 2429264.50, delta_t: 24.040 }, // Year 1939.00
    DeltaTValue{jd: 2429447.00, delta_t: 24.200 }, // Year 1939.50
    DeltaTValue{jd: 2429629.50, delta_t: 24.350 }, // Year 1940.00
    DeltaTValue{jd: 2429812.50, delta_t: 24.610 }, // Year 1940.50
    DeltaTValue{jd: 2429995.50, delta_t: 24.820 }, // Year 1941.00
    DeltaTValue{jd: 2430178.00, delta_t: 25.090 }, // Year 1941.50
    DeltaTValue{jd: 2430360.50, delta_t: 25.300 }, // Year 1942.00
    DeltaTValue{jd: 2430543.00, delta_t: 25.560 }, // Year 1942.50
    DeltaTValue{jd: 2430725.50, delta_t: 25.770 }, // Year 1943.00
    DeltaTValue{jd: 2430908.00, delta_t: 26.050 }, // Year 1943.50
    DeltaTValue{jd: 2431090.50, delta_t: 26.270 }, // Year 1944.00
    DeltaTValue{jd: 2431273.50, delta_t: 26.540 }, // Year 1944.50
    DeltaTValue{jd: 2431456.50, delta_t: 26.760 }, // Year 1945.00
    DeltaTValue{jd: 2431639.00, delta_t: 27.040 }, // Year 1945.50
    DeltaTValue{jd: 2431821.50, delta_t: 27.270 }, // Year 1946.00
    DeltaTValue{jd: 2432004.00, delta_t: 27.550 }, // Year 1946.50
    DeltaTValue{jd: 2432186.50, delta_t: 27.770 }, // Year 1947.00
    DeltaTValue{jd: 2432369.00, delta_t: 28.030 }, // Year 1947.50
    DeltaTValue{jd: 2432551.50, delta_t: 28.250 }, // Year 1948.00
    DeltaTValue{jd: 2432734.50, delta_t: 28.500 }, // Year 1948.50
    DeltaTValue{jd: 2432917.50, delta_t: 28.700 }, // Year 1949.00
    DeltaTValue{jd: 2433100.00, delta_t: 28.950 }, // Year 1949.50
    DeltaTValue{jd: 2433282.50, delta_t: 29.150 }, // Year 1950.00
    DeltaTValue{jd: 2433465.00, delta_t: 29.380 }, // Year 1950.50
    DeltaTValue{jd: 2433647.50, delta_t: 29.570 }, // Year 1951.00
    DeltaTValue{jd: 2433830.00, delta_t: 29.800 }, // Year 1951.50
    DeltaTValue{jd: 2434012.50, delta_t: 29.970 }, // Year 1952.00
    DeltaTValue{jd: 2434195.50, delta_t: 30.190 }, // Year 1952.50
    DeltaTValue{jd: 2434378.50, delta_t: 30.360 }, // Year 1953.00
    DeltaTValue{jd: 2434561.00, delta_t: 30.570 }, // Year 1953.50
    DeltaTValue{jd: 2434743.50, delta_t: 30.720 }, // Year 1954.00
    DeltaTValue{jd: 2434926.00, delta_t: 30.930 }, // Year 1954.50
    DeltaTValue{jd: 2435108.50, delta_t: 31.070 }, // Year 1955.00
    DeltaTValue{jd: 2435291.00, delta_t: 31.240 }, // Year 1955.50
    DeltaTValue{jd: 2435473.50, delta_t: 31.349 }, // Year 1956.00
    DeltaTValue{jd: 2435656.50, delta_t: 31.516 }, // Year 1956.50
    DeltaTValue{jd: 2435839.50, delta_t: 31.677 }, // Year 1957.00
    DeltaTValue{jd: 2436022.00, delta_t: 31.923 }, // Year 1957.50
    DeltaTValue{jd: 2436204.50, delta_t: 32.166 }, // Year 1958.00
    DeltaTValue{jd: 2436387.00, delta_t: 32.449 }, // Year 1958.50
    DeltaTValue{jd: 2436569.50, delta_t: 32.671 }, // Year 1959.00
    DeltaTValue{jd: 2436752.00, delta_t: 32.919 }, // Year 1959.50
    DeltaTValue{jd: 2436934.50, delta_t: 33.150 }, // Year 1960.00
    DeltaTValue{jd: 2437117.50, delta_t: 33.397 }, // Year 1960.50
    DeltaTValue{jd: 2437300.50, delta_t: 33.584 }, // Year 1961.00
    DeltaTValue{jd: 2437483.00, delta_t: 33.804 }, // Year 1961.50
    DeltaTValue{jd: 2437665.50, delta_t: 33.992 }, // Year 1962.00
    DeltaTValue{jd: 2437848.00, delta_t: 34.240 }, // Year 1962.50
    DeltaTValue{jd: 2438030.50, delta_t: 34.466 }, // Year 1963.00
    DeltaTValue{jd: 2438213.00, delta_t: 34.731 }, // Year 1963.50
    DeltaTValue{jd: 2438395.50, delta_t: 35.030 }, // Year 1964.00
    DeltaTValue{jd: 2438578.50, delta_t: 35.400 }, // Year 1964.50
    DeltaTValue{jd: 2438761.50, delta_t: 35.738 }, // Year 1965.00
    DeltaTValue{jd: 2438944.00, delta_t: 36.147 }, // Year 1965.50
    DeltaTValue{jd: 2439126.50, delta_t: 36.546 }, // Year 1966.00
    DeltaTValue{jd: 2439309.00, delta_t: 36.995 }, // Year 1966.50
    DeltaTValue{jd: 2439491.50, delta_t: 37.429 }, // Year 1967.00
    DeltaTValue{jd: 2439674.00, delta_t: 37.879 }, // Year 1967.50
    DeltaTValue{jd: 2439856.50, delta_t: 38.291 }, // Year 1968.00
    DeltaTValue{jd: 2440039.50, delta_t: 38.753 }, // Year 1968.50
    DeltaTValue{jd: 2440222.50, delta_t: 39.204 }, // Year 1969.00
    DeltaTValue{jd: 2440405.00, delta_t: 39.707 }, // Year 1969.50
    DeltaTValue{jd: 2440587.50, delta_t: 40.182 }, // Year 1970.00
    DeltaTValue{jd: 2440770.00, delta_t: 40.706 }, // Year 1970.50
    DeltaTValue{jd: 2440952.50, delta_t: 41.170 }, // Year 1971.00
    DeltaTValue{jd: 2441135.00, delta_t: 41.686 }, // Year 1971.50
    DeltaTValue{jd: 2441317.50, delta_t: 42.227 }, // Year 1972.00
    DeltaTValue{jd: 2441500.50, delta_t: 42.825 }, // Year 1972.50
];

/// Calculate the correction delta_t between UT1 and TT, i.e.
/// TT - UT1 = delta_t
/// In: Julian Day in UTC
/// Out: delta_t, in seconds
fn delta_t(jd: f64) -> f64 {

}

/// Convert UT1 to T(erestial) T(ime)
/// In: Julian Day
fn ut1_to_tt(jd: f64) -> f64 {
    jd
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn cumulative_leap_seconds_test1() {
        // Arrange
        let jd = jd::from_date_hms(2003, 8, 28, 3, 17, 0.0);

        // Act
        let leap_seconds = cumulative_leap_seconds(jd);

        // Assert
        assert_approx_eq!(32.0, leap_seconds, 0.1)
    }

    #[test]
    fn hour_angle_test() {
        // Meeus, page 95, example 13.b

        // Arrange
        let siderial_time_apparent_greenwich = Degrees::from_hms(8, 34, 56.853);
        let longitude_observer = Degrees::from_hms(5, 8, 15.7);
        let right_ascension_apparent = Degrees::from_hms(23, 9, 16.641);

        // Act
        let siderial_time_local =
            local_siderial_time(siderial_time_apparent_greenwich, longitude_observer);
        let hour_angle = hour_angle(siderial_time_local, right_ascension_apparent);

        // Assert
        assert_approx_eq!(64.352133, hour_angle.0, 0.00001)
    }

    #[test]
    fn local_siderial_time_test_1() {
        // Arrange

        // SS: Jan 29th, 2022, 2:32:20pm UTC
        let jd = 2_459_609.105793;

        let longitude_observer = Degrees::from_dms(105, 12, 53.8);

        let mean_siderial_time = mean_siderial_time(jd);

        // Act
        let theta0 = local_siderial_time(mean_siderial_time, longitude_observer);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 16);
        assert_eq!(m, 6);
        assert_approx_eq!(46.9, s, 0.1)
    }

    #[test]
    fn mean_siderial_time_test_1() {
        // Arrange

        // SS: Jan 16th, 2022, 2:26:18pm UTC
        let jd = 2_459_596.101598;

        // Act
        let theta0 = mean_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 22);
        assert_eq!(m, 10);
        assert_approx_eq!(19.92073, s, 0.00001)
    }

    #[test]
    fn mean_siderial_time_test_2() {
        // Meeus, example 12.b, page 89

        // Arrange

        // SS: Apr. 10th 1987, 19h:21m:00s UT
        let jd = 2_446_896.30625;

        // Act
        let theta0 = mean_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_approx_eq!(128.7378734, theta0.0, 0.00001);

        assert_eq!(h, 8);
        assert_eq!(m, 34);
        assert_approx_eq!(57.0896, s, 0.0001)
    }

    #[test]
    fn apparent_siderial_time_test_1() {
        // Arrange

        // SS: Jan 16th, 2022, 2:26:18pm UTC
        let jd = 2_459_596.101598;

        // Act
        let theta0 = apparent_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 22);
        assert_eq!(m, 10);
        assert_approx_eq!(19.10356, s, 0.00001)
    }

    #[test]
    fn apparent_siderial_time_test_2() {
        // Meeus, example 12.a, page 88

        // Arrange

        // SS: Apr. 10th 1987, 0 UT
        let jd = 2_446_895.5;

        // Act
        let theta0 = apparent_siderial_time(jd);
        let (h, m, s) = theta0.to_hms();

        // Assert
        assert_eq!(h, 13);
        assert_eq!(m, 10);
        assert_approx_eq!(46.1351, s, 0.000_1)
    }
}
