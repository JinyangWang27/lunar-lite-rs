use crate::normalize::{normalize_lunar_date, validate_lunar_basic_shape};
use crate::{LunarDate, LunarError, SolarDate};
use crate::{
    calendar::{add_days, days_between, validate_solar_date},
    year_info::{resolve_lunar_year, solar_date_in_lunar_year, year_info},
};

pub fn solar_to_lunar(date: SolarDate) -> Result<LunarDate, LunarError> {
    validate_solar_date(date)?;

    let lunar_year = resolve_lunar_year(date)?;
    let info = year_info(lunar_year)?;

    if !solar_date_in_lunar_year(date, info)? {
        return Err(LunarError::YearOutOfRange { year: date.year });
    }

    let mut offset = days_between(info.new_year, date)?;

    for index in 0..info.month_count as usize {
        let month_days = info.month_days[index] as i32;

        if offset < month_days {
            let month_code = info.month_codes[index];

            return Ok(LunarDate {
                year: lunar_year,
                month: month_code.unsigned_abs(),
                day: (offset + 1) as u8,
                is_leap_month: month_code < 0,
            });
        }

        offset -= month_days;
    }
    Err(LunarError::YearOutOfRange { year: lunar_year })
}

pub fn lunar_to_solar(date: LunarDate) -> Result<SolarDate, LunarError> {
    validate_lunar_basic_shape(date)?;

    let date = normalize_lunar_date(date)?;
    let info = year_info(date.year)?;

    let target_month_code = if date.is_leap_month {
        -(date.month as i8)
    } else {
        date.month as i8
    };

    let mut offset = 0i32;

    for index in 0..info.month_count as usize {
        let month_code = info.month_codes[index];
        let month_days = info.month_days[index];

        if month_code == target_month_code {
            if date.day > month_days {
                return Err(LunarError::InvalidLunarDate {
                    year: date.year,
                    month: date.month,
                    day: date.day,
                    is_leap_month: date.is_leap_month,
                });
            }

            return add_days(info.new_year, offset + date.day as i32 - 1);
        }

        offset += month_days as i32;
    }

    Err(LunarError::InvalidLunarDate {
        year: date.year,
        month: date.month,
        day: date.day,
        is_leap_month: date.is_leap_month,
    })
}
