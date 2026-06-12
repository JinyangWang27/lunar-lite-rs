//! Solar-term (节气) boundary data for the four-pillar month and year pillars.
//!
//! The runtime never computes astronomical solar terms; it reads the generated
//! [`crate::generated::solar_terms`] table (sourced from `lunar-typescript@1.8.6`).

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
