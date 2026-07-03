use thiserror::Error;

/// Errors from date conversion and validation.
///
/// This enum is `#[non_exhaustive]`: new variants may be added in future
/// releases, so downstream `match` expressions should include a wildcard arm.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LunarError {
    /// The solar date is not a real calendar date.
    #[error("Invalid solar date: {year:04}-{month:02}-{day:02}")]
    InvalidSolarDate { year: i32, month: u8, day: u8 },
    /// The lunar date does not exist in that lunar year.
    #[error("Invalid lunar date: {year:04}-{month:02}-{day:02}, is_leap_month={is_leap_month}")]
    InvalidLunarDate {
        year: i32,
        month: u8,
        day: u8,
        is_leap_month: bool,
    },
    /// The lunar month itself is invalid: the month number is outside `1..=12`,
    /// or a leap instance was requested for a month that has none that year.
    #[error("Invalid lunar month: {year:04}-{month:02}, is_leap_month={is_leap_month}")]
    InvalidLunarMonth {
        year: i32,
        month: u8,
        is_leap_month: bool,
    },
    /// The solar year falls outside the supported range (`1..=9999`).
    ///
    /// Also returned when a lunar-to-solar conversion produces a solar date
    /// outside that range.
    #[error("Solar year {year} is out of supported range")]
    SolarYearOutOfRange { year: i32 },
    /// The lunar year falls outside the supported range (`-1..=9999`).
    #[error("Lunar year {year} is out of supported range")]
    LunarYearOutOfRange { year: i32 },
    /// The time is not a valid 24-hour wall-clock time.
    #[error("Invalid time: {hour:02}:{minute:02}")]
    InvalidTime { hour: u8, minute: u8 },
    /// The time index is outside the valid `0..=12` range (时辰 index).
    #[error("Invalid time index: {time_index} (expected 0..=12)")]
    InvalidTimeIndex { time_index: u8 },
}

/// Errors from constructing a [`StemBranch`](crate::StemBranch).
///
/// This enum is `#[non_exhaustive]`: new variants may be added in future
/// releases, so downstream `match` expressions should include a wildcard arm.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StemBranchError {
    /// The stem and branch do not share a position in the sexagenary cycle
    /// (their indices have different parity).
    #[error("Invalid stem-branch pair: {stem:?} - {branch:?}")]
    InvalidStemBranchPair {
        stem: crate::stem_branch::HeavenlyStem,
        branch: crate::stem_branch::EarthlyBranch,
    },
}
