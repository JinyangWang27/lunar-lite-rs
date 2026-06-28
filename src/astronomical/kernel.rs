//! Internal astronomical calculation kernel for new-moon and solar-term offsets.
//!
//! This module holds the numerical routines `lunar-lite` needs to locate new
//! moons (朔) and solar terms (节气) relative to the J2000 epoch. Portions of the
//! kernel are adapted from MIT-licensed `6tail/tyme4rs`; see
//! `THIRD_PARTY_LICENSES.md`.

#![allow(clippy::approx_constant, clippy::needless_late_init)]

use crate::astronomical::data::{
    DELTA_T_TABLE, EARTH_LONGITUDE_TERMS, MOON_LONGITUDE_TERMS, NUTATION_TERMS, QI_CALIBRATION,
    QI_CORRECTIONS, SHUO_CALIBRATION, SHUO_CORRECTIONS,
};
use std::f64::consts::PI;

static PI_2: f64 = PI * 2.0;
static ONE_THIRD: f64 = 1.0 / 3.0;
static SECOND_PER_DAY: f64 = 86400.0;
static SECOND_PER_RAD: f64 = 180.0 * 3600.0 / PI;

/// Internal astronomical calculation engine for new-moon and solar-term offsets.
#[derive(Debug)]
pub struct AstronomicalKernel {}

impl AstronomicalKernel {
    fn approx_nutation_longitude(t: f64) -> f64 {
        let mut a: f64 = -1.742 * t;
        let t2: f64 = t * t;
        let mut dl: f64 = 0.0;
        let mut i: usize = 0;
        let size: usize = NUTATION_TERMS.len();
        while i < size {
            dl += (NUTATION_TERMS[i + 3] + a)
                * (NUTATION_TERMS[i] + NUTATION_TERMS[i + 1] * t + NUTATION_TERMS[i + 2] * t2)
                    .sin();
            a = 0.0;
            i += 5;
        }
        dl / 100.0 / SECOND_PER_RAD
    }

    fn earth_ecliptic_longitude(pt: f64, n: isize) -> f64 {
        let t: f64 = pt / 10.0;
        let mut v: f64 = 0.0;
        let mut tn: f64 = 1.0;
        let mut m: usize;
        let pn: usize = 1;
        let m0: f64 = EARTH_LONGITUDE_TERMS[pn + 1] - EARTH_LONGITUDE_TERMS[pn];
        for i in 0..6 {
            let n1: usize = EARTH_LONGITUDE_TERMS[pn + i] as usize;
            let n2: usize = EARTH_LONGITUDE_TERMS[pn + 1 + i] as usize;
            let n0: f64 = (n2 - n1) as f64;
            if n0 == 0.0 {
                continue;
            }
            if n < 0 {
                m = n2;
            } else {
                m = ((3.0 * (n as f64) * n0 / m0 + 0.5) as usize) + n1;
                if i != 0 {
                    m += 3;
                }
                if m > n2 {
                    m = n2;
                }
            }
            let mut c: f64 = 0.0;
            let mut j: usize = n1;
            while j < m {
                c += EARTH_LONGITUDE_TERMS[j]
                    * (EARTH_LONGITUDE_TERMS[j + 1] + t * EARTH_LONGITUDE_TERMS[j + 2]).cos();
                j += 3;
            }
            v += c * tn;
            tn *= t;
        }
        v /= EARTH_LONGITUDE_TERMS[0];
        let t2: f64 = t * t;
        v += (-0.0728 - 2.7702 * t - 1.1019 * t2 - 0.0996 * t2 * t) / SECOND_PER_RAD;
        v
    }

    fn moon_ecliptic_longitude(t: f64, pn: isize) -> f64 {
        let obl: isize = MOON_LONGITUDE_TERMS[0].len() as isize;
        let mut tn: f64 = 1.0;
        let mut v: f64 = 0.0;
        let mut t2: f64 = t * t;
        let mut t3: f64 = t2 * t;
        let mut t4: f64 = t3 * t;
        let t5: f64 = t4 * t;
        let tx: f64 = t - 10.0;
        v += (3.81034409 + 8399.684730072 * t - 3.319e-05 * t2 + 3.11e-08 * t3 - 2.033e-10 * t4)
            * SECOND_PER_RAD;
        v += 5028.792262 * t + 1.1124406 * t2 + 0.00007699 * t3
            - 0.000023479 * t4
            - 0.0000000178 * t5;
        if tx > 0.0 {
            v += -0.866 + 1.43 * tx + 0.054 * tx * tx;
        }
        t2 /= 1e4;
        t3 /= 1e8;
        t4 /= 1e8;

        let mut n: isize = pn * 6;
        if n < 0 {
            n = obl;
        }
        for (i, f) in MOON_LONGITUDE_TERMS.iter().enumerate() {
            let l: usize = f.len();
            let mut m: usize = (((n * (l as isize) / obl) as f64) + 0.5) as usize;
            if i > 0 {
                m += 6;
            }
            if m >= l {
                m = l;
            }
            let mut c: f64 = 0.0;
            let mut j: usize = 0;
            while j < m {
                c += f[j]
                    * (f[j + 1] + t * f[j + 2] + t2 * f[j + 3] + t3 * f[j + 4] + t4 * f[j + 5])
                        .cos();
                j += 6;
            }
            v += c * tn;
            tn *= t;
        }
        v /= SECOND_PER_RAD;
        v
    }

    fn sun_aberration_longitude(t: f64) -> f64 {
        let t2: f64 = t * t;
        let v: f64 = -0.043126 + 628.301955 * t - 0.000002732 * t2;
        let e: f64 = 0.016708634 - 0.000042037 * t - 0.0000001267 * t2;
        -20.49552 * (1.0 + e * v.cos()) / SECOND_PER_RAD
    }

    fn earth_longitude_velocity(t: f64) -> f64 {
        let f: f64 = 628.307585 * t;
        628.332
            + 21.0 * (1.527 + f).sin()
            + 0.44 * (1.48 + f * 2.0).sin()
            + 0.129 * (5.82 + f).sin() * t
            + 0.00055 * (4.21 + f).sin() * t * t
    }

    fn solar_apparent_longitude(t: f64, n: isize) -> f64 {
        Self::earth_ecliptic_longitude(t, n)
            + Self::approx_nutation_longitude(t)
            + Self::sun_aberration_longitude(t)
            + PI
    }

    fn delta_t_extrapolated(y: f64, jsd: f64) -> f64 {
        let dy: f64 = (y - 1820.0) / 100.0;
        -20.0 + jsd * dy * dy
    }

    fn delta_t_seconds(y: f64) -> f64 {
        let size: usize = DELTA_T_TABLE.len();
        let y0: f64 = DELTA_T_TABLE[size - 2];
        let t0: f64 = DELTA_T_TABLE[size - 1];
        if y >= y0 {
            let jsd: f64 = 31.0;
            if y > y0 + 100.0 {
                return Self::delta_t_extrapolated(y, jsd);
            }
            return Self::delta_t_extrapolated(y, jsd)
                - (Self::delta_t_extrapolated(y0, jsd) - t0) * (y0 + 100.0 - y) / 100.0;
        }
        let mut i: usize = 0;
        while i < size {
            if y < DELTA_T_TABLE[i + 5] {
                break;
            }
            i += 5;
        }
        let t1: f64 = (y - DELTA_T_TABLE[i]) / (DELTA_T_TABLE[i + 5] - DELTA_T_TABLE[i]) * 10.0;
        let t2: f64 = t1 * t1;
        let t3: f64 = t2 * t1;
        DELTA_T_TABLE[i + 1]
            + DELTA_T_TABLE[i + 2] * t1
            + DELTA_T_TABLE[i + 3] * t2
            + DELTA_T_TABLE[i + 4] * t3
    }

    fn delta_t_days(t: f64) -> f64 {
        Self::delta_t_seconds(t / 365.2425 + 2000.0) / SECOND_PER_DAY
    }

    fn moon_longitude_velocity(t: f64) -> f64 {
        let mut v: f64 = 8399.71 - 914.0 * (0.7848 + 8328.691425 * t + 0.0001523 * t * t).sin();
        v -= 179.0 * (2.543 + 15542.7543 * t).sin()
            + 160.0 * (0.1874 + 7214.0629 * t).sin()
            + 62.0 * (3.14 + 16657.3828 * t).sin()
            + 34.0 * (4.827 + 16866.9323 * t).sin()
            + 22.0 * (4.9 + 23871.4457 * t).sin()
            + 12.0 * (2.59 + 14914.4523 * t).sin()
            + 7.0 * (0.23 + 6585.7609 * t).sin()
            + 5.0 * (0.9 + 25195.624 * t).sin()
            + 5.0 * (2.32 - 7700.3895 * t).sin()
            + 5.0 * (3.88 + 8956.9934 * t).sin()
            + 5.0 * (0.49 + 7771.3771 * t).sin();
        v
    }

    fn solar_apparent_longitude_time(w: f64) -> f64 {
        let mut v: f64 = 628.3319653318;
        let mut t = (w - 1.75347 - PI) / v;
        v = Self::earth_longitude_velocity(t);
        t += (w - Self::solar_apparent_longitude(t, 10)) / v;
        v = Self::earth_longitude_velocity(t);
        t += (w - Self::solar_apparent_longitude(t, -1)) / v;
        t
    }

    fn moon_sun_longitude_difference(t: f64, mn: isize, sn: isize) -> f64 {
        Self::moon_ecliptic_longitude(t, mn) + (-3.4E-6)
            - (Self::earth_ecliptic_longitude(t, sn) + Self::sun_aberration_longitude(t) + PI)
    }

    fn new_moon_longitude_alignment_time(w: f64) -> f64 {
        let mut v: f64 = 7771.37714500204;
        let mut t: f64 = (w + 1.08472) / v;
        t += (w - Self::moon_sun_longitude_difference(t, 3, 3)) / v;
        v = Self::moon_longitude_velocity(t) - Self::earth_longitude_velocity(t);
        t += (w - Self::moon_sun_longitude_difference(t, 20, 10)) / v;
        t += (w - Self::moon_sun_longitude_difference(t, -1, 60)) / v;
        t
    }

    fn approx_solar_apparent_longitude_time(w: f64) -> f64 {
        let v: f64 = 628.3319653318;
        let mut t: f64 = (w - 1.75347 - PI) / v;
        t -= (0.000005297 * t * t
            + 0.0334166 * (4.669257 + 628.307585 * t).cos()
            + 0.0002061 * (2.67823 + 628.307585 * t).cos() * t)
            / v;
        t += (w - Self::earth_ecliptic_longitude(t, 8) - PI
            + (20.5 + 17.2 * (2.1824 - 33.75705 * t).sin()) / SECOND_PER_RAD)
            / v;
        t
    }

    fn approx_new_moon_longitude_alignment_time(w: f64) -> f64 {
        let mut v: f64 = 7771.37714500204;
        let mut t: f64 = (w + 1.08472) / v;
        let mut t2: f64 = t * t;
        t -= (-0.00003309 * t2
            + 0.10976 * (0.784758 + 8328.6914246 * t + 0.000152292 * t2).cos()
            + 0.02224 * (0.18740 + 7214.0628654 * t - 0.00021848 * t2).cos()
            - 0.03342 * (4.669257 + 628.307585 * t).cos())
            / v;
        t2 = t * t;
        let l: f64 = Self::moon_ecliptic_longitude(t, 20)
            - (4.8950632
                + 628.3319653318 * t
                + 0.000005297 * t2
                + 0.0334166 * (4.669257 + 628.307585 * t).cos()
                + 0.0002061 * (2.67823 + 628.307585 * t).cos() * t
                + 0.000349 * (4.6261 + 1256.61517 * t).cos()
                - 20.5 / SECOND_PER_RAD);
        v = 7771.38
            - 914.0 * (0.7848 + 8328.691425 * t + 0.0001523 * t2).sin()
            - 179.0 * (2.543 + 15542.7543 * t).sin()
            - 160.0 * (0.1874 + 7214.0629 * t).sin();
        t += (w - l) / v;
        t
    }

    fn high_precision_solar_term_offset(w: f64) -> f64 {
        let mut t: f64 = Self::approx_solar_apparent_longitude_time(w) * 36525.0;
        t = t - Self::delta_t_days(t) + ONE_THIRD;
        let v: f64 = ((t + 0.5) % 1.0) * SECOND_PER_DAY;
        if v < 1200.0 || v > (SECOND_PER_DAY - 1200.0) {
            t = Self::solar_apparent_longitude_time(w) * 36525.0 - Self::delta_t_days(t)
                + ONE_THIRD;
        }
        t
    }

    fn high_precision_new_moon_offset(w: f64) -> f64 {
        let mut t: f64 = Self::approx_new_moon_longitude_alignment_time(w) * 36525.0;
        t = t - Self::delta_t_days(t) + ONE_THIRD;
        let v: f64 = ((t + 0.5) % 1.0) * SECOND_PER_DAY;
        if v < 1800.0 || v > (SECOND_PER_DAY - 1800.0) {
            t = Self::new_moon_longitude_alignment_time(w) * 36525.0 - Self::delta_t_days(t)
                + ONE_THIRD;
        }
        t
    }

    fn low_precision_solar_term_offset(w: f64) -> f64 {
        let v: f64 = 628.3319653318;
        let mut t: f64 = (w - 4.895062166) / v;
        t -= (53.0 * t * t
            + 334116.0 * (4.67 + 628.307585 * t).cos()
            + 2061.0 * (2.678 + 628.3076 * t).cos() * t)
            / v
            / 10000000.0;
        let n: f64 = 48950621.66
            + 6283319653.318 * t
            + 53.0 * t * t
            + 334166.0 * (4.669257 + 628.307585 * t).cos()
            + 3489.0 * (4.6261 + 1256.61517 * t).cos()
            + 2060.6 * (2.67823 + 628.307585 * t).cos() * t
            - 994.0
            - 834.0 * (2.1824 - 33.75705 * t).sin();
        t -= (n / 10000000.0 - w) / 628.332
            + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
        t * 36525.0 + ONE_THIRD
    }

    fn low_precision_new_moon_offset(w: f64) -> f64 {
        let v: f64 = 7771.37714500204;
        let mut t: f64 = (w + 1.08472) / v;
        t -= (-0.0000331 * t * t
            + 0.10976 * (0.785 + 8328.6914 * t).cos()
            + 0.02224 * (0.187 + 7214.0629 * t).cos()
            - 0.03342 * (4.669 + 628.3076 * t).cos())
            / v
            + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
        t * 36525.0 + ONE_THIRD
    }

    /// Returns the day offset of the solar term (`is_qi`) or new moon nearest
    /// Julian Day `julian_day`, from the analytic series. `is_high` selects the
    /// high-precision variant; `pc` is the table-alignment correction.
    fn analytic_event_offset(is_qi: bool, is_high: bool, julian_day: f64, pc: f64) -> f64 {
        let target_angle: f64;
        if is_qi {
            target_angle = ((julian_day + pc - 2451259.0) / 365.2422 * 24.0).floor() * PI / 12.0;
        } else {
            target_angle = ((julian_day + pc - 2451551.0) / 29.5306).floor() * PI_2;
        }
        let day_offset: f64;
        if is_qi {
            if is_high {
                day_offset = Self::high_precision_solar_term_offset(target_angle);
            } else {
                day_offset = Self::low_precision_solar_term_offset(target_angle);
            }
        } else {
            if is_high {
                day_offset = Self::high_precision_new_moon_offset(target_angle);
            } else {
                day_offset = Self::low_precision_new_moon_offset(target_angle);
            }
        }
        (day_offset + 0.5).floor()
    }

    /// Returns the day offset from J2000 of a calendar event (solar term when
    /// `is_qi`, otherwise new moon), using the calibrated low-precision tables
    /// where available and falling back to the high-precision series elsewhere.
    fn calendar_event_offset(
        is_qi: bool,
        day_offset: f64,
        calibration_table: &[f64],
        pc: f64,
        corrections: &str,
    ) -> f64 {
        let size: usize = calibration_table.len();
        let mut result_offset: f64 = 0.0;
        // Julian Day from the J2000 day offset.
        let julian_day: f64 = day_offset + 2451545.0;
        let table_start_jd: f64 = calibration_table[0] - pc;
        let table_end_jd: f64 = calibration_table[size - 1] - pc;
        if julian_day < table_start_jd || julian_day >= 2436935.0 {
            result_offset = Self::analytic_event_offset(is_qi, true, julian_day, pc);
        } else if julian_day >= table_start_jd && julian_day < table_end_jd {
            let mut i: usize = 0;
            while i < size {
                if julian_day + pc < calibration_table[i + 2] {
                    break;
                }
                i += 2;
            }
            result_offset = (calibration_table[i]
                + calibration_table[i + 1]
                    * ((julian_day + pc - calibration_table[i]) / calibration_table[i + 1])
                        .floor()
                + 0.5)
                .floor();
            if !is_qi && result_offset == 1683460.0 {
                result_offset += 1.0;
            }
            result_offset -= 2451545.0;
        } else if julian_day >= table_end_jd {
            result_offset = Self::analytic_event_offset(is_qi, false, julian_day, pc);
            let correction_index: usize;
            if is_qi {
                correction_index = ((day_offset - table_end_jd) / 365.2422 * 24.0) as usize;
            } else {
                correction_index = ((day_offset - table_end_jd) / 29.5306) as usize;
            }
            let correction: &str = &corrections[correction_index..correction_index + 1];
            if correction == "1" {
                result_offset += 1.0;
            } else if correction == "2" {
                result_offset -= 1.0;
            }
        }
        result_offset
    }

    /// Calculates the new-moon (朔) day offset from J2000 nearest `jd`.
    pub(crate) fn new_moon_day_offset(jd: f64) -> f64 {
        Self::calendar_event_offset(false, jd, &SHUO_CALIBRATION, 14.0, SHUO_CORRECTIONS)
    }

    /// Calculates the solar-term (节气) day offset from J2000 nearest `jd`.
    pub(crate) fn solar_term_day_offset(jd: f64) -> f64 {
        Self::calendar_event_offset(true, jd, &QI_CALIBRATION, 7.0, QI_CORRECTIONS)
    }

    /// Returns the accurate solar-term day offset from J2000 for solar
    /// longitude `w`, applying the ΔT correction.
    fn accurate_solar_term_offset(w: f64) -> f64 {
        let t: f64 = Self::solar_apparent_longitude_time(w) * 36525.0;
        t - Self::delta_t_days(t) + ONE_THIRD
    }

    /// Refines a cursory solar-term day offset `jd` to an accurate Julian-day
    /// offset, nudging by one term step if the first estimate lands on a
    /// neighbouring term.
    pub(crate) fn refine_solar_term_offset(jd: f64) -> f64 {
        let d: f64 = PI / 12.0;
        let w: f64 = ((jd + 293.0) / 365.2422 * 24.0).floor() * d;
        let a: f64 = Self::accurate_solar_term_offset(w);
        if a - jd > 5.0 {
            return Self::accurate_solar_term_offset(w - d);
        }
        if a - jd < -5.0 {
            return Self::accurate_solar_term_offset(w + d);
        }
        a
    }
}
