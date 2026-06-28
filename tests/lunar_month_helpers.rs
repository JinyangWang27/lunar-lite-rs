use lunar_lite::{
    LunarDate, LunarError, has_leap_month, leap_month, lunar_month_days, lunar_to_solar,
    validate_lunar_date,
};

fn lunar(year: i32, month: u8, day: u8, is_leap_month: bool) -> LunarDate {
    LunarDate {
        year,
        month,
        day,
        is_leap_month,
    }
}

#[test]
fn leap_month_returns_known_leap_month() {
    assert_eq!(leap_month(2020).unwrap(), Some(4));
}

#[test]
fn leap_month_returns_none_for_non_leap_year() {
    assert_eq!(leap_month(2024).unwrap(), None);
}

#[test]
fn has_leap_month_matches_leap_month_presence() {
    for year in [2020, 2023, 2024] {
        assert_eq!(
            has_leap_month(year).unwrap(),
            leap_month(year).unwrap().is_some()
        );
    }
}

#[test]
fn lunar_month_days_regular_month_returns_29_or_30() {
    let days = lunar_month_days(2024, 1, false).unwrap();
    assert!([29, 30].contains(&days));
}

#[test]
fn lunar_month_days_leap_month_returns_29_or_30() {
    let days = lunar_month_days(2020, 4, true).unwrap();
    assert!([29, 30].contains(&days));
}

#[test]
fn lunar_month_days_rejects_invalid_month_zero() {
    assert_eq!(
        lunar_month_days(2024, 0, false).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 0,
            day: 1,
            is_leap_month: false,
        }
    );
}

#[test]
fn lunar_month_days_rejects_invalid_month_thirteen() {
    assert_eq!(
        lunar_month_days(2024, 13, false).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 13,
            day: 1,
            is_leap_month: false,
        }
    );
}

#[test]
fn lunar_month_days_rejects_invalid_leap_month_flag_for_non_leap_year() {
    assert_eq!(
        lunar_month_days(2024, 1, true).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 1,
            day: 1,
            is_leap_month: true,
        }
    );
}

#[test]
fn lunar_month_days_rejects_leap_flag_for_wrong_month() {
    assert_eq!(
        lunar_month_days(2020, 5, true).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2020,
            month: 5,
            day: 1,
            is_leap_month: true,
        }
    );
}

#[test]
fn validate_lunar_date_accepts_valid_regular_date() {
    assert_eq!(validate_lunar_date(lunar(2024, 1, 1, false)), Ok(()));
}

#[test]
fn validate_lunar_date_accepts_valid_leap_date() {
    assert_eq!(validate_lunar_date(lunar(2020, 4, 1, true)), Ok(()));
}

#[test]
fn validate_lunar_date_rejects_day_zero() {
    assert_eq!(
        validate_lunar_date(lunar(2024, 1, 0, false)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 1,
            day: 0,
            is_leap_month: false,
        }
    );
}

#[test]
fn validate_lunar_date_rejects_day_30_for_short_month() {
    let short_month = (1..=12)
        .find(|&month| lunar_month_days(2020, month, false).unwrap() == 29)
        .expect("test year should contain a short month");

    assert_eq!(
        validate_lunar_date(lunar(2020, short_month, 30, false)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2020,
            month: short_month,
            day: 30,
            is_leap_month: false,
        }
    );
}

#[test]
fn validate_lunar_date_rejects_leap_date_when_month_is_not_leap() {
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

// --- Lower-boundary (lunar year -1) policy -------------------------------
//
// leap_month and lunar_month_days are calendar facts and remain available for
// the earliest supported lunar year (-1). Full lunar->solar conversion is only
// promised when the resulting solar date lands in solar years 1..=9999; every
// lunar year -1 date falls before solar year 1, so it reports YearOutOfRange.

#[test]
fn leap_month_supported_at_lower_boundary() {
    assert_eq!(leap_month(-1).unwrap(), Some(11));
}

#[test]
fn lunar_month_days_supported_at_lower_boundary() {
    let days = lunar_month_days(-1, 1, false).unwrap();
    assert!([29, 30].contains(&days));
}

#[test]
fn lunar_to_solar_lower_boundary_is_out_of_solar_range() {
    assert_eq!(
        lunar_to_solar(lunar(-1, 1, 1, false)).unwrap_err(),
        LunarError::YearOutOfRange { year: -1 }
    );
}

#[test]
fn lunar_month_days_rejects_below_range_year() {
    assert_eq!(
        lunar_month_days(-2, 1, false).unwrap_err(),
        LunarError::YearOutOfRange { year: -2 }
    );
}

#[test]
fn lunar_month_days_rejects_above_range_year() {
    assert_eq!(
        lunar_month_days(10_000, 1, false).unwrap_err(),
        LunarError::YearOutOfRange { year: 10_000 }
    );
}

#[test]
fn lunar_month_days_are_always_29_or_30_for_supported_years() {
    for year in 1850..=2150 {
        for month in 1..=12 {
            let days = lunar_month_days(year, month, false).unwrap();
            assert!([29, 30].contains(&days), "{year}-{month} had {days} days");

            match leap_month(year).unwrap() {
                Some(leap) if leap == month => {
                    let leap_days = lunar_month_days(year, month, true).unwrap();
                    assert!(
                        [29, 30].contains(&leap_days),
                        "{year} leap {month} had {leap_days} days"
                    );
                }
                _ => assert!(lunar_month_days(year, month, true).is_err()),
            }
        }
    }
}
