/// A solar-calendar date using tyme-compatible Julian/Gregorian reform semantics.
///
/// Conversion APIs support solar years `1..=9999`. Dates before
/// `1582-10-15` use Julian-calendar semantics. The historical Gregorian reform
/// gap `1582-10-05..=1582-10-14` is invalid, matching tyme4rs.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SolarDate {
    /// Solar-calendar year.
    pub year: i32,
    /// Month of the year, `1..=12`.
    pub month: u8,
    /// Day of the month, `1..=31` depending on month and year.
    pub day: u8,
}

/// A date in the Chinese lunar calendar.
///
/// Lunar-month fact APIs support lunar years `-1..=9999`. `lunar_to_solar`
/// additionally requires the resulting solar date to fall within supported
/// solar years `1..=9999`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LunarDate {
    /// Lunar year.
    pub year: i32,
    /// Lunar month, `1..=12`.
    pub month: u8,
    /// Day of the lunar month, `1..=30`.
    pub day: u8,
    /// Whether this is the leap (intercalary) instance of `month`.
    pub is_leap_month: bool,
}
