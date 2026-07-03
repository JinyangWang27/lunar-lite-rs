use crate::astronomical::lunar_month::LunarMonth;
use crate::calendar::{days_between, validate_solar_date};
use crate::julian_day::to_solar_date;
use crate::normalize::normalize_lunar_date;
use crate::{LunarDate, LunarError, SolarDate};

/// Converts a Gregorian (solar) date to its Chinese lunar date.
///
/// # Errors
///
/// Returns [`LunarError::InvalidSolarDate`] if `date` is not a real calendar
/// date, or [`LunarError::SolarYearOutOfRange`] if it falls outside solar years
/// `1..=9999`.
pub fn solar_to_lunar(date: SolarDate) -> Result<LunarDate, LunarError> {
    validate_solar_date(date)?;

    let mut month = LunarMonth::from_ym(date.year, date.month as i8)?;
    let mut offset = days_between(month.first_solar_date()?, date)?;

    while offset < 0 {
        month = month.next(-1)?;
        offset += month.day_count()? as i32;
    }

    Ok(LunarDate {
        year: month.year(),
        month: month.month(),
        day: (offset + 1) as u8,
        is_leap_month: month.is_leap(),
    })
}

/// Converts a Chinese lunar date to its Gregorian (solar) date.
///
/// The input is first normalized: an `is_leap_month` flag on a month that has
/// no leap instance in that year is dropped (see [`normalize_lunar_date`]).
///
/// # Errors
///
/// Returns [`LunarError::InvalidLunarDate`] if the date does not exist,
/// [`LunarError::LunarYearOutOfRange`] if the lunar year is outside `-1..=9999`,
/// or [`LunarError::SolarYearOutOfRange`] if the resulting solar date is outside
/// solar years `1..=9999`.
pub fn lunar_to_solar(date: LunarDate) -> Result<SolarDate, LunarError> {
    let date = normalize_lunar_date(date)?;
    let month_with_leap = if date.is_leap_month {
        -(date.month as i8)
    } else {
        date.month as i8
    };
    let month = LunarMonth::from_ym(date.year, month_with_leap)?;

    // `normalize_lunar_date` above already validates that `date.day` does not
    // exceed this same month's day count, so no further day-bound check is
    // needed here.
    let solar = to_solar_date(month.first_julian_day()? + date.day as f64 - 1.0);
    validate_solar_date(solar)?;
    Ok(solar)
}
