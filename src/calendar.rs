// src/calendar.rs

use crate::{LunarError, SolarDate};

pub(crate) fn validate_solar_date(date: SolarDate) -> Result<(), LunarError> {
    if date.month < 1 || date.month > 12 {
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

    let start_days = days_from_civil(start.year, start.month, start.day);
    let end_days = days_from_civil(end.year, end.month, end.day);

    Ok(end_days - start_days)
}

pub(crate) fn add_days(date: SolarDate, days: i32) -> Result<SolarDate, LunarError> {
    validate_solar_date(date)?;

    let base = days_from_civil(date.year, date.month, date.day);
    Ok(civil_from_days(base + days))
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
        2 if is_gregorian_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_gregorian_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

// Days since 1970-01-01, proleptic Gregorian calendar.
// Algorithm by Howard Hinnant.
pub(crate) fn days_from_civil(year: i32, month: u8, day: u8) -> i32 {
    let mut y = year;
    let m = month as i32;
    let d = day as i32;

    y -= if m <= 2 { 1 } else { 0 };

    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = m + if m > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;

    era * 146_097 + doe - 719_468
}

pub(crate) fn civil_from_days(days: i32) -> SolarDate {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };

    y += if m <= 2 { 1 } else { 0 };

    SolarDate {
        year: y,
        month: m as u8,
        day: d as u8,
    }
}
