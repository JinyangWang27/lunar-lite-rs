//! `lunar-lite` is a small library for Chinese lunisolar date conversion.
//!
//! It converts between the Gregorian (solar) calendar and the Chinese lunar
//! calendar, and exposes the sexagenary cycle (六十甲子) of Heavenly Stems and
//! Earthly Branches.
//!
//! # Examples
//!
//! ```
//! use lunar_lite::{solar_to_lunar, lunar_to_solar, SolarDate, LunarDate};
//!
//! let solar = SolarDate { year: 2024, month: 2, day: 10 };
//! let lunar = solar_to_lunar(solar).unwrap();
//! assert_eq!(lunar, LunarDate { year: 2024, month: 1, day: 1, is_leap_month: false });
//!
//! // Round-trips back to the original solar date.
//! assert_eq!(lunar_to_solar(lunar).unwrap(), solar);
//! ```
//!
//! # Supported range
//!
//! Conversions use an internal astronomical new-moon and solar-term backend
//! adapted from the MIT-licensed `6tail/tyme4rs` ShouXing routines.
//!
//! - Solar conversion supports Gregorian years `1..=9999`.
//! - Lunar conversion supports lunar years `-1..=9999`.
//! - The historical Gregorian reform gap `1582-10-05..=1582-10-14` is invalid.
//!
//! Years outside the supported ranges return [`LunarError::YearOutOfRange`].
//!
//! # Lunar month helpers
//!
//! `leap_month`, `has_leap_month`, `lunar_month_days`, and
//! `validate_lunar_date` expose calendar facts only. They do not encode
//! downstream chart-placement policy for how consumers should interpret leap
//! months. Invalid month and leap-month selections return
//! [`LunarError::InvalidLunarDate`].
//!
//! # Features
//!
//! - `serde`: derive `Serialize`/`Deserialize` for the public date and
//!   stem-branch types.

mod astronomical;
mod calendar;
mod convert;
mod date;
mod error;
mod four_pillars;
mod julian_day;
mod lunar_month;
mod normalize;
mod sexagenary;
mod solar_terms;
mod stem_branch;
mod time_index;

pub use convert::{lunar_to_solar, solar_to_lunar};
pub use date::{LunarDate, SolarDate};
pub use error::{LunarError, StemBranchError};
pub use four_pillars::{
    FourPillars, HeavenlyStemAndEarthlyBranchDate, MonthDivide, StemBranchOptions, YearDivide,
    four_pillars_from_solar_date, four_pillars_from_solar_date_with_options,
    get_heavenly_stem_and_earthly_branch_by_solar_date,
    get_heavenly_stem_and_earthly_branch_by_solar_date_with_options,
};
pub use lunar_month::{has_leap_month, leap_month, lunar_month_days, validate_lunar_date};
pub use normalize::normalize_lunar_date;
pub use sexagenary::{StemBranch, lunar_year_branch, lunar_year_stem, lunar_year_stem_branch};
pub use stem_branch::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};
pub use time_index::{is_early_zi, is_late_zi, time_index, time_index_to_branch};
