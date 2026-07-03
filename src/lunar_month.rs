use crate::astronomical::{lunar_month::LunarMonth, lunar_year};
use crate::{LunarDate, LunarError};

/// Returns the leap month number for a lunar year, if that year has one.
///
/// # Errors
///
/// Returns [`LunarError::LunarYearOutOfRange`] if `year` is outside `-1..=9999`.
pub fn leap_month(year: i32) -> Result<Option<u8>, LunarError> {
    lunar_year::leap_month(year)
}

/// Returns whether a lunar year has a leap month.
///
/// # Errors
///
/// Returns [`LunarError::LunarYearOutOfRange`] if `year` is outside `-1..=9999`.
pub fn has_leap_month(year: i32) -> Result<bool, LunarError> {
    Ok(leap_month(year)?.is_some())
}

/// Returns the number of days in a regular or leap lunar month.
///
/// # Errors
///
/// Returns [`LunarError::InvalidLunarMonth`] if the month is outside `1..=12`
/// or the requested leap month does not exist in that year. Returns
/// [`LunarError::LunarYearOutOfRange`] if `year` is outside `-1..=9999`.
pub fn lunar_month_days(year: i32, month: u8, is_leap_month: bool) -> Result<u8, LunarError> {
    if !(1..=12).contains(&month) {
        return Err(invalid_lunar_month_error(year, month, is_leap_month));
    }

    let month_with_leap = if is_leap_month {
        -(month as i8)
    } else {
        month as i8
    };

    LunarMonth::from_ym(year, month_with_leap)?.day_count()
}

/// Validates that a lunar date exists exactly as supplied.
///
/// Unlike [`crate::normalize_lunar_date`], this function does not clear a fake
/// leap-month flag before validation.
///
/// # Errors
///
/// Returns [`LunarError::InvalidLunarDate`] if the month/day shape is invalid,
/// the day exceeds the actual month length, or the leap-month flag does not
/// match the year's leap month. Returns [`LunarError::LunarYearOutOfRange`] if
/// the year is outside `-1..=9999`.
pub fn validate_lunar_date(date: LunarDate) -> Result<(), LunarError> {
    if !(1..=12).contains(&date.month) || date.day == 0 {
        return Err(invalid_lunar_date(date));
    }

    let days =
        lunar_month_days(date.year, date.month, date.is_leap_month).map_err(
            |error| match error {
                LunarError::InvalidLunarMonth { .. } => invalid_lunar_date(date),
                other => other,
            },
        )?;
    if date.day > days {
        return Err(invalid_lunar_date(date));
    }

    Ok(())
}

fn invalid_lunar_month_error(year: i32, month: u8, is_leap_month: bool) -> LunarError {
    LunarError::InvalidLunarMonth {
        year,
        month,
        is_leap_month,
    }
}

fn invalid_lunar_date(date: LunarDate) -> LunarError {
    LunarError::InvalidLunarDate {
        year: date.year,
        month: date.month,
        day: date.day,
        is_leap_month: date.is_leap_month,
    }
}
