//! Error-taxonomy tests for the 2.0 split of the year-range and lunar-month
//! error variants.
//!
//! These pin the semantic boundaries: solar-year failures report
//! `SolarYearOutOfRange`, lunar-year failures report `LunarYearOutOfRange`,
//! month-shape failures report `InvalidLunarMonth`, and full-date failures
//! report `InvalidLunarDate`.

use lunar_lite::{
    LunarDate, LunarError, SolarDate, four_pillars_from_solar_date, leap_month, li_chun_datetime,
    lunar_month_days, lunar_to_solar, solar_to_lunar, validate_lunar_date,
};

fn solar(year: i32, month: u8, day: u8) -> SolarDate {
    SolarDate { year, month, day }
}

fn lunar(year: i32, month: u8, day: u8, is_leap_month: bool) -> LunarDate {
    LunarDate {
        year,
        month,
        day,
        is_leap_month,
    }
}

// --- Solar year out of range ------------------------------------------------

#[test]
fn solar_to_lunar_reports_solar_year_out_of_range() {
    assert_eq!(
        solar_to_lunar(solar(0, 1, 1)).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: 0 }
    );
    assert_eq!(
        solar_to_lunar(solar(10_000, 1, 1)).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: 10_000 }
    );
}

#[test]
fn four_pillars_reports_solar_year_out_of_range() {
    assert_eq!(
        four_pillars_from_solar_date(solar(0, 6, 1), 0).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: 0 }
    );
    assert_eq!(
        four_pillars_from_solar_date(solar(10_000, 6, 1), 0).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: 10_000 }
    );
}

#[test]
fn li_chun_datetime_reports_solar_year_out_of_range() {
    assert_eq!(
        li_chun_datetime(0).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: 0 }
    );
    assert_eq!(
        li_chun_datetime(10_000).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: 10_000 }
    );
}

#[test]
fn lunar_to_solar_result_before_solar_year_1_reports_solar_year_out_of_range() {
    // Lunar year -1 is a valid lunar-month fact, but its dates land before
    // solar year 1, so the conversion result is out of solar range.
    assert_eq!(leap_month(-1).unwrap(), Some(11));
    assert_eq!(
        lunar_to_solar(lunar(-1, 1, 1, false)).unwrap_err(),
        LunarError::SolarYearOutOfRange { year: -1 }
    );
}

// --- Lunar year out of range ------------------------------------------------

#[test]
fn lunar_month_days_reports_lunar_year_out_of_range() {
    assert_eq!(
        lunar_month_days(-2, 1, false).unwrap_err(),
        LunarError::LunarYearOutOfRange { year: -2 }
    );
    assert_eq!(
        lunar_month_days(10_000, 1, false).unwrap_err(),
        LunarError::LunarYearOutOfRange { year: 10_000 }
    );
}

#[test]
fn lunar_to_solar_reports_lunar_year_out_of_range() {
    assert_eq!(
        lunar_to_solar(lunar(-2, 1, 1, false)).unwrap_err(),
        LunarError::LunarYearOutOfRange { year: -2 }
    );
    assert_eq!(
        lunar_to_solar(lunar(10_000, 1, 1, false)).unwrap_err(),
        LunarError::LunarYearOutOfRange { year: 10_000 }
    );
}

// --- Invalid lunar month (month-shape failures) -----------------------------

#[test]
fn lunar_month_days_reports_invalid_lunar_month() {
    assert_eq!(
        lunar_month_days(2024, 0, false).unwrap_err(),
        LunarError::InvalidLunarMonth {
            year: 2024,
            month: 0,
            is_leap_month: false,
        }
    );
    assert_eq!(
        lunar_month_days(2024, 13, false).unwrap_err(),
        LunarError::InvalidLunarMonth {
            year: 2024,
            month: 13,
            is_leap_month: false,
        }
    );
    // Leap instance requested for a month that has none that year.
    assert_eq!(
        lunar_month_days(2024, 1, true).unwrap_err(),
        LunarError::InvalidLunarMonth {
            year: 2024,
            month: 1,
            is_leap_month: true,
        }
    );
}

// --- Invalid lunar date (full-date failures) --------------------------------

#[test]
fn validate_lunar_date_reports_invalid_lunar_date_for_full_date_failures() {
    // Day exceeds a real month's length.
    let short_month = (1..=12)
        .find(|&m| lunar_month_days(2020, m, false).unwrap() == 29)
        .expect("2020 has a 29-day month");
    assert_eq!(
        validate_lunar_date(lunar(2020, short_month, 30, false)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2020,
            month: short_month,
            day: 30,
            is_leap_month: false,
        }
    );

    // Leap flag on a non-leap month still reports the full date, not the month.
    assert_eq!(
        validate_lunar_date(lunar(2020, 5, 1, true)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2020,
            month: 5,
            day: 1,
            is_leap_month: true,
        }
    );
}

// --- #[non_exhaustive] ------------------------------------------------------

/// Downstream code must be able to match `LunarError` with a wildcard arm; this
/// compiles only because the enum is `#[non_exhaustive]`-friendly.
#[test]
fn lunar_error_is_matchable_with_wildcard() {
    let err = solar_to_lunar(solar(0, 1, 1)).unwrap_err();
    let described = match err {
        LunarError::SolarYearOutOfRange { year } => format!("solar year {year}"),
        _ => "other".to_string(),
    };
    assert_eq!(described, "solar year 0");
}
