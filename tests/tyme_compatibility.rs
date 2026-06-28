use lunar_lite::{
    LunarDate, LunarError, SolarDate, leap_month, lunar_month_days, lunar_to_solar, solar_to_lunar,
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

#[test]
fn solar_to_lunar_matches_tyme_chinese_new_year_edges() {
    let cases = [
        (solar(2024, 2, 9), lunar(2023, 12, 30, false)),
        (solar(2024, 2, 10), lunar(2024, 1, 1, false)),
        (solar(2024, 2, 11), lunar(2024, 1, 2, false)),
    ];

    for (input, expected) in cases {
        assert_eq!(solar_to_lunar(input).unwrap(), expected, "{input:?}");
    }
}

#[test]
fn conversions_match_tyme_leap_month_edges() {
    let solar_cases = [
        (solar(2020, 5, 22), lunar(2020, 4, 30, false)),
        (solar(2020, 5, 23), lunar(2020, 4, 1, true)),
        (solar(2020, 6, 20), lunar(2020, 4, 29, true)),
        (solar(2020, 6, 21), lunar(2020, 5, 1, false)),
    ];

    for (input, expected) in solar_cases {
        assert_eq!(solar_to_lunar(input).unwrap(), expected, "{input:?}");
    }

    let lunar_cases = [
        (lunar(2020, 4, 30, false), solar(2020, 5, 22)),
        (lunar(2020, 4, 1, true), solar(2020, 5, 23)),
        (lunar(2020, 4, 29, true), solar(2020, 6, 20)),
        (lunar(2020, 5, 1, false), solar(2020, 6, 21)),
    ];

    for (input, expected) in lunar_cases {
        assert_eq!(lunar_to_solar(input).unwrap(), expected, "{input:?}");
    }
}

#[test]
fn lunar_month_lengths_match_tyme_short_and_long_months() {
    assert_eq!(lunar_month_days(2023, 6, false).unwrap(), 29);
    assert_eq!(lunar_month_days(2023, 7, false).unwrap(), 30);
    assert_eq!(lunar_month_days(2023, 8, false).unwrap(), 30);
    assert_eq!(lunar_month_days(2023, 9, false).unwrap(), 29);
}

#[test]
fn conversions_match_tyme_modern_round_trip_anchors() {
    let cases = [
        (solar(1900, 1, 31), lunar(1900, 1, 1, false)),
        (solar(1900, 2, 1), lunar(1900, 1, 2, false)),
        (solar(1950, 1, 1), lunar(1949, 11, 13, false)),
        (solar(2000, 2, 5), lunar(2000, 1, 1, false)),
        (solar(2050, 12, 31), lunar(2050, 11, 18, false)),
        (solar(2100, 2, 9), lunar(2100, 1, 1, false)),
    ];

    for (solar_date, lunar_date) in cases {
        assert_eq!(solar_to_lunar(solar_date).unwrap(), lunar_date, "{solar_date:?}");
        assert_eq!(lunar_to_solar(lunar_date).unwrap(), solar_date, "{lunar_date:?}");
    }
}

#[test]
fn conversions_match_tyme_2033_regressions() {
    let cases = [
        (solar(2033, 12, 22), lunar(2033, 11, 1, true)),
        (solar(2034, 1, 20), lunar(2033, 12, 1, false)),
        (solar(2034, 2, 19), lunar(2034, 1, 1, false)),
    ];

    for (solar_date, lunar_date) in cases {
        assert_eq!(solar_to_lunar(solar_date).unwrap(), lunar_date, "{solar_date:?}");
        assert_eq!(lunar_to_solar(lunar_date).unwrap(), solar_date, "{lunar_date:?}");
    }
}

#[test]
fn solar_to_lunar_matches_tyme_1582_reform_policy() {
    assert_eq!(
        solar_to_lunar(solar(1582, 10, 4)).unwrap(),
        lunar(1582, 9, 18, false)
    );
    assert_eq!(
        solar_to_lunar(solar(1582, 10, 5)).unwrap_err(),
        LunarError::InvalidSolarDate {
            year: 1582,
            month: 10,
            day: 5,
        }
    );
    assert_eq!(
        solar_to_lunar(solar(1582, 10, 14)).unwrap_err(),
        LunarError::InvalidSolarDate {
            year: 1582,
            month: 10,
            day: 14,
        }
    );
    assert_eq!(
        solar_to_lunar(solar(1582, 10, 15)).unwrap(),
        lunar(1582, 9, 19, false)
    );
}

#[test]
fn solar_to_lunar_rejects_tyme_unsupported_solar_years() {
    assert_eq!(
        solar_to_lunar(solar(0, 1, 1)).unwrap_err(),
        LunarError::YearOutOfRange { year: 0 }
    );
    assert_eq!(
        solar_to_lunar(solar(10_000, 1, 1)).unwrap_err(),
        LunarError::YearOutOfRange { year: 10_000 }
    );
}

#[test]
fn lunar_helpers_support_tyme_lunar_year_boundaries() {
    assert_eq!(leap_month(-1).unwrap(), Some(11));
    assert_eq!(leap_month(9_999).unwrap(), None);

    assert_eq!(
        lunar_month_days(-2, 1, false).unwrap_err(),
        LunarError::YearOutOfRange { year: -2 }
    );
    assert_eq!(
        lunar_month_days(10_000, 1, false).unwrap_err(),
        LunarError::YearOutOfRange { year: 10_000 }
    );
}

#[test]
fn lunar_to_solar_matches_tyme_lunar_boundary_policy() {
    assert_eq!(
        lunar_to_solar(lunar(9_999, 12, 2, false)).unwrap(),
        solar(9_999, 12, 31)
    );

    assert_eq!(
        lunar_to_solar(lunar(-2, 1, 1, false)).unwrap_err(),
        LunarError::YearOutOfRange { year: -2 }
    );
    assert_eq!(
        lunar_to_solar(lunar(10_000, 1, 1, false)).unwrap_err(),
        LunarError::YearOutOfRange { year: 10_000 }
    );
}
