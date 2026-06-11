use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LunarError {
    #[error("Invalid solar date: {year:04}-{month:02}-{day:02}")]
    InvalidSolarDate { year: i32, month: u8, day: u8 },
    #[error("Invalid lunar date: {year:04}-{month:02}-{day:02}, is_leap_month={is_leap_month}")]
    InvalidLunarDate {
        year: i32,
        month: u8,
        day: u8,
        is_leap_month: bool,
    },
    #[error("Year {year} is out of supported range")]
    YearOutOfRange { year: i32 },
    #[error("Invalid time: {hour:02}:{minute:02}")]
    InvalidTime { hour: u8, minute: u8 },
}
