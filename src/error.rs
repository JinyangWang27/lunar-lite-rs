use thiserror::Error;

/// Errors from date conversion and validation.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
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
    /// The year falls outside the supported conversion range.
    #[error("Year {year} is out of supported range")]
    YearOutOfRange { year: i32 },
    /// The time is not a valid 24-hour wall-clock time.
    #[error("Invalid time: {hour:02}:{minute:02}")]
    InvalidTime { hour: u8, minute: u8 },
    /// The time index is outside the valid `0..=12` range (时辰 index).
    #[error("Invalid time index: {time_index} (expected 0..=12)")]
    InvalidTimeIndex { time_index: u8 },
    /// The Gregorian year falls outside the supported solar-term range.
    #[error("Year {year} is outside the supported solar-term range")]
    SolarTermOutOfRange { year: i32 },
}

/// Errors from constructing a [`StemBranch`](crate::StemBranch).
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum StemBranchError {
    /// The stem and branch do not share a position in the sexagenary cycle
    /// (their indices have different parity).
    #[error("Invalid stem-branch pair: {stem:?} - {branch:?}")]
    InvalidStemBranchPair {
        stem: crate::stem_branch::HeavenlyStem,
        branch: crate::stem_branch::EarthlyBranch,
    },
}
