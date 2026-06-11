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
//! Conversions are backed by a generated table of per-year data; years outside
//! the supported range return [`LunarError::YearOutOfRange`].
//!
//! # Features
//!
//! - `serde`: derive `Serialize`/`Deserialize` for the public date and
//!   stem-branch types.

mod calendar;
mod convert;
mod date;
mod error;
mod generated;
mod normalize;
mod sexagenary;
mod stem_branch;
mod time_index;
mod year_info;

pub use convert::{lunar_to_solar, solar_to_lunar};
pub use date::{LunarDate, SolarDate};
pub use error::{LunarError, StemBranchError};
pub use normalize::normalize_lunar_date;
pub use sexagenary::StemBranch;
pub use stem_branch::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};
pub use time_index::time_index;
