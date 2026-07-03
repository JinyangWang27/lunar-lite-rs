// src/calendar.rs

use crate::julian_day::{from_ymd_hms, is_gregorian_reform_date};
use crate::{LunarError, SolarDate};

const MIN_SOLAR_YEAR: i32 = 1;
const MAX_SOLAR_YEAR: i32 = 9_999;

pub(crate) fn validate_solar_date(date: SolarDate) -> Result<(), LunarError> {
    if !(MIN_SOLAR_YEAR..=MAX_SOLAR_YEAR).contains(&date.year) {
        return Err(LunarError::YearOutOfRange { year: date.year });
    }

    if date.month < 1 || date.month > 12 {
        return Err(invalid_solar(date));
    }

    if date.year == 1582 && date.month == 10 && (5..=14).contains(&date.day) {
        return Err(invalid_solar(date));
    }

    let max_day = days_in_month(date.year, date.month);

    if date.day < 1 || date.day > max_day {
        return Err(invalid_solar(date));
    }

    Ok(())
}

pub(crate) fn days_between(start: SolarDate, end: SolarDate) -> Result<i32, LunarError> {
    validate_solar_date(start)?;
    validate_solar_date(end)?;

    let start_days = from_ymd_hms(start.year, start.month, start.day, 0, 0, 0);
    let end_days = from_ymd_hms(end.year, end.month, end.day, 0, 0, 0);

    Ok((end_days - start_days) as i32)
}

fn invalid_solar(date: SolarDate) -> LunarError {
    LunarError::InvalidSolarDate {
        year: date.year,
        month: date.month,
        day: date.day,
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    // Whether Feb 29 of `year` falls before the Gregorian reform decides
    // which leap-year rule applies.
    if !is_gregorian_reform_date(year, 2, 29) {
        return year % 4 == 0;
    }
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
