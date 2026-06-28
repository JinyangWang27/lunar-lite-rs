//! Astronomical solar-term (节气) lookups for four-pillar month and year pillars.

use crate::astronomical::solar_term;
use crate::calendar::days_from_civil;
use crate::error::LunarError;

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
pub(crate) fn day_instant(year: i32, month: u8, day: u8, second_of_day: i64) -> i64 {
    days_from_civil(year, month, day) as i64 * SECONDS_PER_DAY + second_of_day
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
