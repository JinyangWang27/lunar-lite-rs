/// A date in the Gregorian (solar) calendar.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SolarDate {
    /// Proleptic Gregorian year (may be negative).
    pub year: i32,
    /// Month of the year, `1..=12`.
    pub month: u8,
    /// Day of the month, `1..=31` depending on month and year.
    pub day: u8,
}

/// A date in the Chinese lunar calendar.
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
