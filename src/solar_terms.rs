//! Astronomical solar-term (节气) lookups for four-pillar month and year pillars.

use crate::astronomical::solar_term;
use crate::error::LunarError;
use crate::julian_day::from_ymd_hms;

/// Seconds in a day; used to build absolute second-of-range instants.
pub(crate) const SECONDS_PER_DAY: i64 = 86_400;

/// First Gregorian year supported by conversion APIs.
pub(crate) const MIN_YEAR: i32 = 1;
/// Last Gregorian year supported by conversion APIs.
pub(crate) const MAX_YEAR: i32 = 9_999;

/// Index of 立春 (LiChun) within a year's 12 ordered Jie.
pub(crate) const LI_CHUN: usize = 1;

/// The month earthly-branch index produced by each of the 12 Jie, in table order
/// (小寒→丑, 立春→寅, ..., 立冬→亥, 大雪→子).
pub(crate) const MONTH_BRANCH_OF_JIE: [usize; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0];

/// The branch index for dates before the year's first Jie (小寒): still 子.
pub(crate) const MONTH_BRANCH_BEFORE_FIRST_JIE: usize = 0;

/// Absolute instant of a wall-clock time, comparable against term instants.
///
/// Built on the Julian Day so user-date instants and Jie solar-term instants
/// share one Julian/Gregorian calendar policy (Julian before 1582-10-15,
/// Gregorian after). The Julian Day is rounded to whole seconds to keep
/// comparisons free of floating-point equality artefacts.
pub(crate) fn day_instant(year: i32, month: u8, day: u8, second_of_day: i64) -> i64 {
    let julian_day = from_ymd_hms(year, month, day, 0, 0, 0);
    (julian_day * SECONDS_PER_DAY as f64).round() as i64 + second_of_day
}

/// The 12 Jie instants (seconds since epoch) for `year`, in table order.
pub(crate) fn jie_instants(year: i32) -> Result<[i64; 12], LunarError> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(LunarError::SolarTermOutOfRange { year });
    }

    let mut out = [0i64; 12];
    for (index, slot) in out.iter_mut().enumerate() {
        let term_index = index as i32 * 2 + 1;
        let term = solar_term::term_datetime(year, term_index);
        *slot = day_instant(
            term.date.year,
            term.date.month,
            term.date.day,
            term.hour as i64 * 3600 + term.minute as i64 * 60 + term.second as i64,
        );
    }
    Ok(out)
}

/// The `(month, day)` on which 立春 falls in `year`.
pub(crate) fn li_chun_date(year: i32) -> Result<(u8, u8), LunarError> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(LunarError::SolarTermOutOfRange { year });
    }

    let term = solar_term::term_datetime(year, 3);
    Ok((term.date.month, term.date.day))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn li_chun_dates_match_reference() {
        assert_eq!(li_chun_date(2000).unwrap(), (2, 4));
        assert_eq!(li_chun_date(2024).unwrap(), (2, 4));
    }

    #[test]
    fn jie_instants_are_strictly_increasing() {
        let instants = jie_instants(2000).unwrap();
        for pair in instants.windows(2) {
            assert!(pair[0] < pair[1]);
        }
    }

    #[test]
    fn li_chun_2000_instant_matches_tyme_reference() {
        let instants = jie_instants(2000).unwrap();
        let expected = day_instant(2000, 2, 4, 20 * 3600 + 40 * 60 + 24);
        assert_eq!(instants[LI_CHUN], expected);
    }

    #[test]
    fn pre_reform_julian_leap_day_does_not_collapse() {
        // 1500 is a Julian leap year, so 1500-02-29 is a real, distinct day that
        // must not collapse onto 1500-03-01 the way a proleptic-Gregorian day
        // count would. Instants must be consecutive and strictly increasing.
        let feb28 = day_instant(1500, 2, 28, 0);
        let feb29 = day_instant(1500, 2, 29, 0);
        let mar01 = day_instant(1500, 3, 1, 0);

        assert!(feb28 < feb29 && feb29 < mar01);
        assert_eq!(feb29 - feb28, SECONDS_PER_DAY);
        assert_eq!(mar01 - feb29, SECONDS_PER_DAY);
    }

    #[test]
    fn out_of_range_years_error() {
        assert_eq!(
            jie_instants(0).unwrap_err(),
            LunarError::SolarTermOutOfRange { year: 0 }
        );
        assert_eq!(
            li_chun_date(10_000).unwrap_err(),
            LunarError::SolarTermOutOfRange { year: 10_000 }
        );
    }
}
