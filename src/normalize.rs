use crate::year_info::year_info;
use crate::{LunarDate, LunarError};

pub fn normalize_lunar_date(date: LunarDate) -> Result<LunarDate, LunarError> {
    validate_lunar_basic_shape(date)?;

    if !date.is_leap_month {
        return Ok(date);
    }

    let info = year_info(date.year)?;

    if info.leap_month == Some(date.month) {
        Ok(date)
    } else {
        Ok(LunarDate {
            is_leap_month: false,
            ..date
        })
    }
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
