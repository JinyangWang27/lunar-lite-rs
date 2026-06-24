use crate::year_info::year_info;
use crate::{LunarDate, LunarError};

/// Returns the leap month number for a lunar year, if that year has one.
///
/// # Errors
///
/// Returns [`LunarError::YearOutOfRange`] if `year` is outside the generated
/// lunar-year table.
pub fn leap_month(year: i32) -> Result<Option<u8>, LunarError> {
    Ok(year_info(year)?.leap_month)
}

/// Returns whether a lunar year has a leap month.
///
/// # Errors
///
/// Returns [`LunarError::YearOutOfRange`] if `year` is outside the generated
/// lunar-year table.
pub fn has_leap_month(year: i32) -> Result<bool, LunarError> {
    Ok(leap_month(year)?.is_some())
}

/// Returns the number of days in a regular or leap lunar month.
///
/// # Errors
///
/// Returns [`LunarError::InvalidLunarMonth`] if the month is outside `1..=12`
/// or the requested leap month does not exist in that year. Returns
/// [`LunarError::YearOutOfRange`] if `year` is outside the generated lunar-year
/// table.
pub fn lunar_month_days(year: i32, month: u8, is_leap_month: bool) -> Result<u8, LunarError> {
    if !(1..=12).contains(&month) {
        return Err(invalid_lunar_month(year, month, is_leap_month));
    }

    let info = year_info(year)?;
    let target_month_code = if is_leap_month {
        if info.leap_month != Some(month) {
            return Err(invalid_lunar_month(year, month, is_leap_month));
        }

        -(month as i8)
    } else {
        month as i8
    };

    info.month_codes
        .iter()
        .zip(info.month_days.iter())
        .take(info.month_count as usize)
        .find_map(|(&month_code, &days)| (month_code == target_month_code).then_some(days))
        .ok_or_else(|| invalid_lunar_month(year, month, is_leap_month))
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
/// match the year's leap month. Returns [`LunarError::YearOutOfRange`] if the
/// year is outside the generated lunar-year table.
pub fn validate_lunar_date(date: LunarDate) -> Result<(), LunarError> {
    if !(1..=12).contains(&date.month) || date.day == 0 {
        return Err(invalid_lunar_date(date));
    }

    let days = lunar_month_days(date.year, date.month, date.is_leap_month)
        .map_err(|error| map_month_error_to_date_error(error, date))?;
    if date.day > days {
        return Err(invalid_lunar_date(date));
    }

    Ok(())
}

fn invalid_lunar_month(year: i32, month: u8, is_leap_month: bool) -> LunarError {
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

fn map_month_error_to_date_error(error: LunarError, date: LunarDate) -> LunarError {
    match error {
        LunarError::InvalidLunarMonth { .. } => invalid_lunar_date(date),
        other => other,
    }
}
