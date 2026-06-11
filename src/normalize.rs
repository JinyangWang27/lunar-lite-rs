use crate::year_info::year_info;
use crate::{LunarDate, LunarError};

/// Normalizes a lunar date and verifies that it actually exists.
///
/// If `is_leap_month` is set on a month that has no leap instance in that year,
/// the flag is cleared. The day is then checked against that month's length.
///
/// # Errors
///
/// Returns [`LunarError::InvalidLunarDate`] if the month/day are out of shape or
/// the day exceeds the month's length, or [`LunarError::YearOutOfRange`] if the
/// year is outside the supported range.
pub fn normalize_lunar_date(date: LunarDate) -> Result<LunarDate, LunarError> {
    validate_lunar_basic_shape(date)?;

    let info = year_info(date.year)?;

    let normalized = if date.is_leap_month && info.leap_month != Some(date.month) {
        LunarDate {
            is_leap_month: false,
            ..date
        }
    } else {
        date
    };

    validate_lunar_actual_date(normalized)?;

    Ok(normalized)
}
pub(crate) fn validate_lunar_basic_shape(date: LunarDate) -> Result<(), LunarError> {
    if date.month < 1 || date.month > 12 || date.day < 1 || date.day > 30 {
        return Err(LunarError::InvalidLunarDate {
            year: date.year,
            month: date.month,
            day: date.day,
            is_leap_month: date.is_leap_month,
        });
    }

    Ok(())
}

pub(crate) fn validate_lunar_actual_date(date: LunarDate) -> Result<(), LunarError> {
    let info = year_info(date.year)?;

    let target_month_code = if date.is_leap_month {
        -(date.month as i8)
    } else {
        date.month as i8
    };

    for index in 0..info.month_count as usize {
        if info.month_codes[index] == target_month_code {
            if date.day <= info.month_days[index] {
                return Ok(());
            }

            break;
        }
    }

    Err(LunarError::InvalidLunarDate {
        year: date.year,
        month: date.month,
        day: date.day,
        is_leap_month: date.is_leap_month,
    })
}
