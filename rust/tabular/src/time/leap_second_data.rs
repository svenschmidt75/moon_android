use std::cmp::Ordering;

pub struct LeapSecondCoefficient {
    pub jd: f64,
    pub leap_seconds: f64,
    pub base_mjd: f64,
    pub coefficient: f64,
}

impl PartialEq<Self> for LeapSecondCoefficient {
    fn eq(&self, other: &Self) -> bool {
        self.jd == other.jd
    }
}

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

/// Data based on https://cddis.nasa.gov/archive/products/iers/tai-utc.dat
/// This table needs to be updated every few years to take new data into
/// account.
pub const LEAP_SECOND_DATA: [LeapSecondCoefficient; 41] = [
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
