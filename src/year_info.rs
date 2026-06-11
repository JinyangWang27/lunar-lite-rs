use crate::calendar::days_between;
use crate::generated::year_info::{MAX_LUNAR_YEAR, MIN_LUNAR_YEAR, YEAR_INFOS};
use crate::{LunarError, SolarDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LunarYearInfo {
    pub year: i32,
    pub new_year: SolarDate,
    pub leap_month: Option<u8>,
    pub month_count: u8,
    pub month_codes: [i8; 13],
    pub month_days: [u8; 13],
}

pub(crate) fn year_info(year: i32) -> Result<&'static LunarYearInfo, LunarError> {
    if !(MIN_LUNAR_YEAR..=MAX_LUNAR_YEAR).contains(&year) {
        return Err(LunarError::YearOutOfRange { year });
    }
    let index = (year - MIN_LUNAR_YEAR) as usize;
    Ok(&YEAR_INFOS[index])
}

pub(crate) fn resolve_lunar_year(date: SolarDate) -> Result<i32, LunarError> {
    if date.year < MIN_LUNAR_YEAR || date.year > MAX_LUNAR_YEAR {
        return Err(LunarError::YearOutOfRange { year: date.year });
    }

    let current = year_info(date.year)?;

    if date >= current.new_year {
        return Ok(current.year);
    }

    let previous_year = date.year - 1;
    if previous_year < MIN_LUNAR_YEAR {
        return Err(LunarError::YearOutOfRange {
            year: previous_year,
        });
    }

    Ok(previous_year)
}

pub(crate) fn lunar_year_total_days(info: &LunarYearInfo) -> i32 {
    info.month_days
        .iter()
        .take(info.month_count as usize)
        .map(|days| *days as i32)
        .sum()
}

pub(crate) fn solar_date_in_lunar_year(
    date: SolarDate,
    info: &LunarYearInfo,
) -> Result<bool, LunarError> {
    let offset = days_between(info.new_year, date)?;
    Ok(offset >= 0 && offset < lunar_year_total_days(info))
}
