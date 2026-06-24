//! Solar-term (节气) boundary data and lookups for the four-pillar month and year
//! pillars.
//!
//! The runtime never computes astronomical solar terms; it reads the generated
//! [`crate::generated::solar_terms`] table (sourced from `lunar-typescript@1.8.6`).

use crate::calendar::{civil_from_days, days_from_civil, validate_solar_date};
use crate::date::SolarDate;
use crate::error::LunarError;
use crate::generated::solar_terms::{
    JIE_BOUNDARIES, JIE_END_YEAR, JIE_START_YEAR, SOLAR_TERM_BOUNDARIES,
};

/// One of the 24 solar terms (节气).
///
/// These helpers expose the Gregorian calendar date on which a solar term
/// occurs. They do not expose the exact hour/minute/second of the solar term.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SolarTerm {
    /// 立春.
    LiChun,
    /// 雨水.
    YuShui,
    /// 惊蛰.
    JingZhe,
    /// 春分.
    ChunFen,
    /// 清明.
    QingMing,
    /// 谷雨.
    GuYu,
    /// 立夏.
    LiXia,
    /// 小满.
    XiaoMan,
    /// 芒种.
    MangZhong,
    /// 夏至.
    XiaZhi,
    /// 小暑.
    XiaoShu,
    /// 大暑.
    DaShu,
    /// 立秋.
    LiQiu,
    /// 处暑.
    ChuShu,
    /// 白露.
    BaiLu,
    /// 秋分.
    QiuFen,
    /// 寒露.
    HanLu,
    /// 霜降.
    ShuangJiang,
    /// 立冬.
    LiDong,
    /// 小雪.
    XiaoXue,
    /// 大雪.
    DaXue,
    /// 冬至.
    DongZhi,
    /// 小寒.
    XiaoHan,
    /// 大寒.
    DaHan,
}

impl SolarTerm {
    fn table_index(self) -> usize {
        match self {
            Self::LiChun => 0,
            Self::YuShui => 1,
            Self::JingZhe => 2,
            Self::ChunFen => 3,
            Self::QingMing => 4,
            Self::GuYu => 5,
            Self::LiXia => 6,
            Self::XiaoMan => 7,
            Self::MangZhong => 8,
            Self::XiaZhi => 9,
            Self::XiaoShu => 10,
            Self::DaShu => 11,
            Self::LiQiu => 12,
            Self::ChuShu => 13,
            Self::BaiLu => 14,
            Self::QiuFen => 15,
            Self::HanLu => 16,
            Self::ShuangJiang => 17,
            Self::LiDong => 18,
            Self::XiaoXue => 19,
            Self::DaXue => 20,
            Self::DongZhi => 21,
            Self::XiaoHan => 22,
            Self::DaHan => 23,
        }
    }
}

/// A solar-term moment within a Gregorian year, stored compactly.
///
/// `ordinal` is the 1-based day of the Gregorian year (1 == January 1), and
/// `second_of_day` is the number of seconds since `00:00:00` on that day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TermMoment {
    /// 1-based day of the Gregorian year, `1..=366`.
    pub(crate) ordinal: u16,
    /// Seconds since `00:00:00` local time on that day, `0..86_400`.
    pub(crate) second_of_day: u32,
}

/// Seconds in a day; used to build absolute second-of-range instants.
pub(crate) const SECONDS_PER_DAY: i64 = 86_400;

/// First Gregorian year covered by the solar-term table.
pub(crate) const MIN_YEAR: i32 = JIE_START_YEAR;
/// Last Gregorian year covered by the solar-term table.
pub(crate) const MAX_YEAR: i32 = JIE_END_YEAR;

/// Index of 立春 (LiChun) within a year's 12 ordered Jie.
pub(crate) const LI_CHUN: usize = 1;

/// The month earthly-branch index produced by each of the 12 Jie, in table order
/// (小寒→丑, 立春→寅, …, 立冬→亥, 大雪→子).
pub(crate) const MONTH_BRANCH_OF_JIE: [usize; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0];

/// The branch index for dates before the year's first Jie (小寒): still 子.
pub(crate) const MONTH_BRANCH_BEFORE_FIRST_JIE: usize = 0;

fn year_terms(year: i32) -> Result<&'static [TermMoment; 12], LunarError> {
    if !(JIE_START_YEAR..=JIE_END_YEAR).contains(&year) {
        return Err(LunarError::SolarTermOutOfRange { year });
    }
    Ok(&JIE_BOUNDARIES[(year - JIE_START_YEAR) as usize])
}

fn year_solar_terms(year: i32) -> Result<&'static [TermMoment; 24], LunarError> {
    if !(JIE_START_YEAR..=JIE_END_YEAR).contains(&year) {
        return Err(LunarError::SolarTermOutOfRange { year });
    }
    Ok(&SOLAR_TERM_BOUNDARIES[(year - JIE_START_YEAR) as usize])
}

fn term_date(year: i32, moment: TermMoment) -> SolarDate {
    civil_from_days(days_from_civil(year, 1, 1) + (moment.ordinal as i32 - 1))
}

/// Absolute instant of a [`TermMoment`], in seconds since the proleptic-Gregorian
/// epoch (1970-01-01). Comparable against [`day_instant`].
fn term_instant(year: i32, moment: TermMoment) -> i64 {
    let day_number = days_from_civil(year, 1, 1) + (moment.ordinal as i32 - 1);
    day_number as i64 * SECONDS_PER_DAY + moment.second_of_day as i64
}

/// Absolute instant of a wall-clock time, comparable against term instants.
pub(crate) fn day_instant(year: i32, month: u8, day: u8, second_of_day: i64) -> i64 {
    days_from_civil(year, month, day) as i64 * SECONDS_PER_DAY + second_of_day
}

/// The 12 Jie instants (seconds since epoch) for `year`, in table order.
pub(crate) fn jie_instants(year: i32) -> Result<[i64; 12], LunarError> {
    let terms = year_terms(year)?;
    let mut out = [0i64; 12];
    for (slot, &moment) in out.iter_mut().zip(terms.iter()) {
        *slot = term_instant(year, moment);
    }
    Ok(out)
}

/// Returns the Gregorian date on which `term` occurs in `year`.
///
/// This is a date-level helper: it does not expose the exact
/// hour/minute/second of the solar term.
///
/// # Errors
/// Returns [`LunarError::SolarTermOutOfRange`] if `year` is outside the
/// generated solar-term table.
pub fn solar_term_date(year: i32, term: SolarTerm) -> Result<SolarDate, LunarError> {
    let moment = year_solar_terms(year)?[term.table_index()];
    Ok(term_date(year, moment))
}

/// Returns the Gregorian date on which 立春 (LiChun) occurs in `year`.
///
/// This is a date-level helper: it does not expose the exact
/// hour/minute/second of LiChun.
///
/// # Errors
/// Returns [`LunarError::SolarTermOutOfRange`] if `year` is outside the
/// generated solar-term table.
pub fn li_chun_date(year: i32) -> Result<SolarDate, LunarError> {
    solar_term_date(year, SolarTerm::LiChun)
}

/// Returns whether `date` is on or after that Gregorian year's LiChun date.
///
/// This compares calendar dates only.
///
/// # Errors
/// - [`LunarError::InvalidSolarDate`] if `date` is not a real Gregorian date.
/// - [`LunarError::SolarTermOutOfRange`] if `date.year` is outside the
///   generated solar-term table.
pub fn is_on_or_after_li_chun(date: SolarDate) -> Result<bool, LunarError> {
    validate_solar_date(date)?;
    Ok(date >= li_chun_date(date.year)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn li_chun_dates_match_reference() {
        // lunar-typescript@1.8.6: 立春 2000 = 2000-02-04, 2024 = 2024-02-04.
        assert_eq!(
            li_chun_date(2000).unwrap(),
            SolarDate {
                year: 2000,
                month: 2,
                day: 4
            }
        );
        assert_eq!(
            li_chun_date(2024).unwrap(),
            SolarDate {
                year: 2024,
                month: 2,
                day: 4
            }
        );
    }

    #[test]
    fn jie_instants_are_strictly_increasing() {
        let instants = jie_instants(2000).unwrap();
        for pair in instants.windows(2) {
            assert!(pair[0] < pair[1]);
        }
    }

    #[test]
    fn li_chun_2000_instant_matches_reference() {
        // 立春 2000 = 2000-02-04 20:40:24.
        let instants = jie_instants(2000).unwrap();
        let expected = day_instant(2000, 2, 4, 20 * 3600 + 40 * 60 + 24);
        assert_eq!(instants[LI_CHUN], expected);
    }

    #[test]
    fn out_of_range_years_error() {
        assert_eq!(
            jie_instants(1849).unwrap_err(),
            LunarError::SolarTermOutOfRange { year: 1849 }
        );
        assert_eq!(
            li_chun_date(2151).unwrap_err(),
            LunarError::SolarTermOutOfRange { year: 2151 }
        );
    }
}
