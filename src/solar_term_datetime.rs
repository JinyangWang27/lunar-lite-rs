//! Public exact solar-term datetime primitive.
//!
//! Exposes [`SolarTermDateTime`] and [`li_chun_datetime`] as stable public
//! primitives so downstream crates can consume datetime-level 立春 (LiChun)
//! without depending on internal astronomical helpers.

use crate::SolarDate;
use crate::astronomical::solar_term;
use crate::error::LunarError;
use crate::solar_terms::{MAX_YEAR, MIN_YEAR};

/// The exact date and wall-clock time at which a solar term occurs.
///
/// All fields are in local solar time (no time-zone correction).
///
/// # Examples
///
/// ```
/// use lunar_lite::li_chun_datetime;
///
/// let dt = li_chun_datetime(2000).unwrap();
/// assert_eq!(dt.date.year, 2000);
/// assert_eq!(dt.date.month, 2);
/// assert_eq!(dt.date.day, 4);
/// assert_eq!(dt.hour, 20);
/// assert_eq!(dt.minute, 40);
/// assert_eq!(dt.second, 24);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SolarTermDateTime {
    /// Gregorian date on which the solar term falls.
    pub date: SolarDate,
    /// Hour of the solar term (0–23).
    pub hour: u8,
    /// Minute of the solar term (0–59).
    pub minute: u8,
    /// Second of the solar term (0–59).
    pub second: u8,
}

/// Returns the exact datetime of 立春 (LiChun, Start of Spring) for `year`.
///
/// The calculation reuses the internal astronomical backend. The supported
/// range is `1..=9999`; years outside that range return
/// [`LunarError::SolarTermOutOfRange`].
///
/// This function exposes a public primitive for downstream crates that need
/// datetime-level LiChun precision. It does **not** change
/// [`YearDivide::Exact`](crate::YearDivide) semantics in the four-pillar
/// API, which remains date-level for compatibility.
///
/// # Errors
///
/// Returns [`LunarError::SolarTermOutOfRange`] when `year` is outside
/// `1..=9999`.
///
/// # Examples
///
/// ```
/// use lunar_lite::{li_chun_datetime, LunarError};
///
/// let dt = li_chun_datetime(2000).unwrap();
/// assert_eq!(dt.date.year, 2000);
/// assert_eq!(dt.date.month, 2);
/// assert_eq!(dt.date.day, 4);
/// assert_eq!(dt.hour, 20);
/// assert_eq!(dt.minute, 40);
/// assert_eq!(dt.second, 24);
///
/// assert_eq!(
///     li_chun_datetime(0).unwrap_err(),
///     LunarError::SolarTermOutOfRange { year: 0 },
/// );
/// ```
pub fn li_chun_datetime(year: i32) -> Result<SolarTermDateTime, LunarError> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(LunarError::SolarTermOutOfRange { year });
    }
    // Term index 3 = 立春 in Tyme's 24-term order (0=winter solstice, 3=start of spring).
    let term = solar_term::term_datetime(year, 3);
    Ok(SolarTermDateTime {
        date: term.date,
        hour: term.hour,
        minute: term.minute,
        second: term.second,
    })
}
