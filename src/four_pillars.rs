//! Four-pillar (四柱 / BaZi) Heavenly Stem and Earthly Branch calculation.
//!
//! This is a faithful port of the TypeScript `lunar-lite@0.2.8` function
//! `getHeavenlyStemAndEarthlyBranchBySolarDate`, validated against generated
//! fixtures. Given a Gregorian [`SolarDate`], a 时辰 `time_index` (0..=12), and a
//! [`StemBranchOptions`], it returns the year, month, day, and hour pillars.
//!
//! Like the reference, the wall-clock time is synthesized from `time_index` as
//! `hour = max(time_index * 2 - 1, 0), minute = 30`. The supported range is
//! **1850-01-01 ..= 2150-12-31**.
//!
//! ## Year pillar
//! - [`YearDivide::Normal`]: the lunar-year pillar (Chinese New Year boundary).
//! - [`YearDivide::Exact`]: the 立春 (LiChun) boundary, compared at **date**
//!   granularity (matching the reference's `getYearGanByLiChun`).
//!
//! ## Month pillar
//! The month pillar uses solar terms, not the lunar month, in `Exact` mode.
//! - [`MonthDivide::Normal`]: lunar-month 五虎遁 (uses [`solar_to_lunar`]).
//! - [`MonthDivide::Exact`]: the 12 Jie (节) boundaries at **exact second**.

use crate::calendar::validate_solar_date;
use crate::convert::solar_to_lunar;
use crate::date::SolarDate;
use crate::error::LunarError;
use crate::julian_day::day_pillar_offset;
use crate::sexagenary::StemBranch;
use crate::solar_terms::{self, LI_CHUN, MONTH_BRANCH_BEFORE_FIRST_JIE, MONTH_BRANCH_OF_JIE};
use crate::stem_branch::{EarthlyBranch, HeavenlyStem};

/// Highest valid 时辰 index (late 子时).
const MAX_TIME_INDEX: u8 = 12;

/// How to resolve the year pillar across the year boundary.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum YearDivide {
    /// Use the lunar year (Chinese New Year boundary).
    Normal,
    /// Use the 立春 (LiChun) boundary, at date granularity.
    Exact,
}

/// How to resolve the month pillar across the month boundary.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MonthDivide {
    /// Use the lunar month with 五虎遁 (not solar terms).
    Normal,
    /// Use the 12 Jie (节) solar-term boundaries at exact second.
    Exact,
}

/// Options controlling the year and month pillar boundaries.
///
/// The default (`Exact`, `Exact`) matches `lunar-lite@0.2.8`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StemBranchOptions {
    /// Year pillar boundary mode.
    pub year: YearDivide,
    /// Month pillar boundary mode.
    pub month: MonthDivide,
}

impl Default for StemBranchOptions {
    fn default() -> Self {
        Self {
            year: YearDivide::Exact,
            month: MonthDivide::Exact,
        }
    }
}

/// The four pillars (年柱, 月柱, 日柱, 时柱) of a date and time.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HeavenlyStemAndEarthlyBranchDate {
    /// Year pillar (年柱).
    pub yearly: StemBranch,
    /// Month pillar (月柱).
    pub monthly: StemBranch,
    /// Day pillar (日柱).
    pub daily: StemBranch,
    /// Hour pillar (时柱).
    pub hourly: StemBranch,
}

/// Computes the four pillars for a Gregorian solar date and 时辰 index.
///
/// `time_index` is in `0..=12`, where both `0` (early 子) and `12` (late 子) map
/// to the 子 branch; `12` additionally rolls the day pillar to the next day
/// (晚子时), matching the reference.
///
/// # Errors
/// - [`LunarError::InvalidSolarDate`] if `solar` is not a real date.
/// - [`LunarError::InvalidTimeIndex`] if `time_index > 12`.
/// - [`LunarError::SolarTermOutOfRange`] if `solar.year` is outside 1850..=2150.
/// - [`LunarError::YearOutOfRange`] for `Normal` options when the lunar year is
///   outside the table (the early-1850 corner before Chinese New Year 1850).
pub fn get_heavenly_stem_and_earthly_branch_by_solar_date(
    solar: SolarDate,
    time_index: u8,
    options: StemBranchOptions,
) -> Result<HeavenlyStemAndEarthlyBranchDate, LunarError> {
    validate_solar_date(solar)?;

    if time_index > MAX_TIME_INDEX {
        return Err(LunarError::InvalidTimeIndex { time_index });
    }

    if !(solar_terms::MIN_YEAR..=solar_terms::MAX_YEAR).contains(&solar.year) {
        return Err(LunarError::SolarTermOutOfRange { year: solar.year });
    }

    // Synthesized wall-clock time: hour = max(time_index*2 - 1, 0), minute = 30.
    let synth_hour = (time_index as i64 * 2 - 1).max(0);
    let synth_second_of_day = synth_hour * 3600 + 30 * 60;

    let yearly = year_pillar(solar, options.year)?;
    let monthly = match options.month {
        MonthDivide::Normal => month_pillar_normal(solar, yearly)?,
        MonthDivide::Exact => month_pillar_exact(solar, synth_second_of_day)?,
    };

    // Day pillar: floor(noonJulianDay) - 11, rolled forward for late 子时.
    let mut day_offset = day_pillar_offset(solar.year, solar.month, solar.day);
    if time_index == MAX_TIME_INDEX {
        day_offset += 1;
    }
    let daily = StemBranch::from_cycle_index(day_offset.rem_euclid(60) as usize);
    let day_stem_index = day_offset.rem_euclid(10) as usize;

    // Hour pillar: branch from time_index, stem derived from the (rolled) day stem.
    let hour_branch_index = (time_index % 12) as usize;
    let hour_stem_index = (day_stem_index % 5 * 2 + hour_branch_index) % 10;
    let hourly = pillar_from_indices(hour_stem_index, hour_branch_index);

    Ok(HeavenlyStemAndEarthlyBranchDate {
        yearly,
        monthly,
        daily,
        hourly,
    })
}

/// Shorter alias for [`get_heavenly_stem_and_earthly_branch_by_solar_date`].
pub fn solar_date_to_ganzhi(
    solar: SolarDate,
    time_index: u8,
    options: StemBranchOptions,
) -> Result<HeavenlyStemAndEarthlyBranchDate, LunarError> {
    get_heavenly_stem_and_earthly_branch_by_solar_date(solar, time_index, options)
}

fn year_pillar(solar: SolarDate, divide: YearDivide) -> Result<StemBranch, LunarError> {
    match divide {
        YearDivide::Normal => Ok(StemBranch::from_lunar_year(solar_to_lunar(solar)?.year)),
        YearDivide::Exact => {
            let (li_chun_month, li_chun_day) = solar_terms::li_chun_date(solar.year)?;
            let before_li_chun = (solar.month, solar.day) < (li_chun_month, li_chun_day);
            let pillar_year = if before_li_chun {
                solar.year - 1
            } else {
                solar.year
            };
            // Sexagenary of a Gregorian year: the 1984 anchor is congruent mod 60.
            Ok(StemBranch::from_lunar_year(pillar_year))
        }
    }
}

fn month_pillar_normal(solar: SolarDate, yearly: StemBranch) -> Result<StemBranch, LunarError> {
    let lunar = solar_to_lunar(solar)?;
    let year_stem = yearly.stem().index();
    let yin_stem = (year_stem % 5 * 2 + 2) % 10;
    // Leap month past its 15th day counts toward the following month.
    let fix_leap = usize::from(lunar.is_leap_month && lunar.day > 15);
    let offset = (lunar.month as usize - 1) + fix_leap;
    let stem = (yin_stem + offset) % 10;
    let branch = (2 + offset) % 12; // lunar month 1 (正月) == 寅 (index 2)
    Ok(pillar_from_indices(stem, branch))
}

fn month_pillar_exact(
    solar: SolarDate,
    synth_second_of_day: i64,
) -> Result<StemBranch, LunarError> {
    let instant = solar_terms::day_instant(solar.year, solar.month, solar.day, synth_second_of_day);
    let jie = solar_terms::jie_instants(solar.year)?;

    // Month branch: the branch of the most recent Jie at or before `instant`.
    let mut branch = MONTH_BRANCH_BEFORE_FIRST_JIE;
    for (k, &boundary) in jie.iter().enumerate() {
        if boundary <= instant {
            branch = MONTH_BRANCH_OF_JIE[k];
        } else {
            break;
        }
    }

    // Month stem by 五虎遁 from the 立春 suì year (exact-second granularity).
    let sui_year = if instant >= jie[LI_CHUN] {
        solar.year
    } else {
        solar.year - 1
    };
    let sui_stem = (sui_year - 4).rem_euclid(10) as usize;
    let yin_stem = (sui_stem % 5 * 2 + 2) % 10;
    let offset_from_yin = (branch + 12 - 2) % 12;
    let stem = (yin_stem + offset_from_yin) % 10;
    Ok(pillar_from_indices(stem, branch))
}

/// Builds a [`StemBranch`] from stem (0..10) and branch (0..12) indices that are
/// guaranteed to share parity by construction.
fn pillar_from_indices(stem_index: usize, branch_index: usize) -> StemBranch {
    StemBranch::try_new(
        HeavenlyStem::from_index(stem_index),
        EarthlyBranch::from_index(branch_index),
    )
    .expect("computed stem and branch share parity by construction")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sb(stem: HeavenlyStem, branch: EarthlyBranch) -> StemBranch {
        StemBranch::try_new(stem, branch).unwrap()
    }

    fn solar(year: i32, month: u8, day: u8) -> SolarDate {
        SolarDate { year, month, day }
    }

    const EXACT: StemBranchOptions = StemBranchOptions {
        year: YearDivide::Exact,
        month: MonthDivide::Exact,
    };
    const NORMAL: StemBranchOptions = StemBranchOptions {
        year: YearDivide::Normal,
        month: MonthDivide::Normal,
    };

    // Reference: lunar-lite@0.2.8, 2000-08-16 timeIndex 2 -> 庚辰 甲申 丙午 庚寅.
    #[test]
    fn spot_check_2000_08_16() {
        let r = get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2000, 8, 16), 2, EXACT)
            .unwrap();
        assert_eq!(r.yearly, sb(HeavenlyStem::Geng, EarthlyBranch::Chen));
        assert_eq!(r.monthly, sb(HeavenlyStem::Jia, EarthlyBranch::Shen));
        assert_eq!(r.daily, sb(HeavenlyStem::Bing, EarthlyBranch::Wu));
        assert_eq!(r.hourly, sb(HeavenlyStem::Geng, EarthlyBranch::Yin));

        // Interior date: normal options agree with exact.
        let n = get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2000, 8, 16), 2, NORMAL)
            .unwrap();
        assert_eq!(n, r);
    }

    // Late 子时: same date, time_index 0 vs 12. Day pillar rolls and the hour stem
    // follows the rolled day stem.
    #[test]
    fn late_zi_rolls_day_and_hour() {
        let early =
            get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2000, 8, 16), 0, EXACT)
                .unwrap();
        assert_eq!(early.daily, sb(HeavenlyStem::Bing, EarthlyBranch::Wu));
        assert_eq!(early.hourly, sb(HeavenlyStem::Wu, EarthlyBranch::Zi));

        let late =
            get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2000, 8, 16), 12, EXACT)
                .unwrap();
        assert_eq!(late.daily, sb(HeavenlyStem::Ding, EarthlyBranch::Wei));
        assert_eq!(late.hourly, sb(HeavenlyStem::Geng, EarthlyBranch::Zi));
    }

    #[test]
    fn all_time_indices_produce_expected_branches() {
        // Branch index for each time_index: 0 and 12 -> 子, otherwise time_index.
        for ti in 0..=12u8 {
            let r =
                get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2000, 8, 16), ti, EXACT)
                    .unwrap();
            let expected = EarthlyBranch::from_index((ti % 12) as usize);
            assert_eq!(r.hourly.branch(), expected, "time_index {ti}");
        }
    }

    #[test]
    fn alias_matches_primary() {
        let a = get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2024, 6, 1), 5, EXACT)
            .unwrap();
        let b = solar_date_to_ganzhi(solar(2024, 6, 1), 5, EXACT).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn invalid_time_index_errors() {
        assert_eq!(
            get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2000, 1, 1), 13, EXACT),
            Err(LunarError::InvalidTimeIndex { time_index: 13 })
        );
    }

    #[test]
    fn year_out_of_range_errors() {
        assert_eq!(
            get_heavenly_stem_and_earthly_branch_by_solar_date(solar(1849, 6, 1), 0, EXACT),
            Err(LunarError::SolarTermOutOfRange { year: 1849 })
        );
        assert_eq!(
            get_heavenly_stem_and_earthly_branch_by_solar_date(solar(2151, 6, 1), 0, EXACT),
            Err(LunarError::SolarTermOutOfRange { year: 2151 })
        );
    }

    #[test]
    fn default_options_are_exact_exact() {
        assert_eq!(
            StemBranchOptions::default(),
            StemBranchOptions {
                year: YearDivide::Exact,
                month: MonthDivide::Exact,
            }
        );
    }
}
