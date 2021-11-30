//! Phase of the moon
use crate::{jd, util};

const SYNODIC_MONTH: f64 = 29.53058868;
const SYNODIC_MONTH_OVER_2: f64 = SYNODIC_MONTH / 2.0;

/// Calculate the phase of the moon.
pub fn phase_angle(jd: f64) -> f64 {
    // SS: JD of a new moon, Jan. 21st, 19:20, 5:25
    let new_moon_jd = jd::from_date(1920, 1, 21, 0.225694444);

    // SS: number of days since new moon
    let delta_jd = jd - new_moon_jd;

    // SS: days into the moon phase from new moon
    let phase_days = delta_jd % SYNODIC_MONTH;

    let phase_angle = if phase_days > 15.0 {
        // SS: past full moon
        SYNODIC_MONTH_OVER_2 - (phase_days % SYNODIC_MONTH_OVER_2)
    } else {
        phase_days
    };

    //    phase_angle / SYNODIC_MONTH * 360.0
    phase_angle / SYNODIC_MONTH_OVER_2 * 100.0
}

pub fn fraction_illuminated(jd: f64) -> f64 {
    let phase_angle = util::to_radians(phase_angle(jd));
    (1.0 + phase_angle.cos()) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn phase_angle_test() {
        // SS: 2021 Nov. 29, 12:33am TD
        let jd = jd::from_date(2021, 11, 29, 0.525);

        // Act
        let phase_angle = phase_angle(jd);

        // Assert
        assert_approx_eq!(63.9091644374556, phase_angle, 0.000_001)
    }

    #[test]
    fn fraction_illuminated_test() {
        // SS: 2021 Nov. 29, 12:33am TD
        let jd = jd::from_date(2021, 11, 29, 0.525);

        // Act
        let fraction_illuminated = fraction_illuminated(jd);

        // Assert
        assert_approx_eq!(1.0 - 0.7198977625352061, fraction_illuminated, 0.000_001)
    }
}
