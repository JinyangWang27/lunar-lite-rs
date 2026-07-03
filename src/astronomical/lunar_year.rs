//! Lunar-year leap-month lookup.
//!
//! The compressed per-year leap-month encoding is adapted from MIT-licensed
//! `6tail/tyme4rs`; see `THIRD_PARTY_LICENSES.md`.

use crate::LunarError;
use crate::astronomical::data::LEAP_MONTH_CODES;

pub(crate) const MIN_LUNAR_YEAR: i32 = -1;
pub(crate) const MAX_LUNAR_YEAR: i32 = 9_999;

pub(crate) fn validate_lunar_year(year: i32) -> Result<(), LunarError> {
    if !(MIN_LUNAR_YEAR..=MAX_LUNAR_YEAR).contains(&year) {
        return Err(LunarError::LunarYearOutOfRange { year });
    }
    Ok(())
}

pub(crate) fn leap_month(year: i32) -> Result<Option<u8>, LunarError> {
    validate_lunar_year(year)?;
    let index = (year - MIN_LUNAR_YEAR) as usize;
    let code = LEAP_MONTH_CODES.as_bytes()[index];
    let month = match code {
        b'0'..=b'9' => code - b'0',
        b'a'..=b'c' => code - b'a' + 10,
        _ => unreachable!("leap-month code is from a fixed adapted encoding"),
    };
    Ok((month != 0).then_some(month))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_leap_months_match_tyme() {
        assert_eq!(leap_month(-1).unwrap(), Some(11));
        assert_eq!(leap_month(2020).unwrap(), Some(4));
        assert_eq!(leap_month(2023).unwrap(), Some(2));
        assert_eq!(leap_month(2024).unwrap(), None);
        assert_eq!(leap_month(2033).unwrap(), Some(11));
        assert_eq!(leap_month(9_999).unwrap(), None);
    }
}
