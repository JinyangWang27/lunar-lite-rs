use lunar_lite::{LunarDate, LunarError, SolarDate, solar_to_lunar};

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

// --- Solar date validation through the public conversion API ---------------

#[test]
fn solar_to_lunar_rejects_invalid_gregorian_dates() {
    let cases = [
        ("month 0", solar(2024, 0, 1)),
        ("month 13", solar(2024, 13, 1)),
        ("day 0", solar(2024, 1, 0)),
        ("April 31", solar(2024, 4, 31)),
        ("Feb 29 in non-leap year", solar(2023, 2, 29)),
    ];

    for (label, date) in cases {
        assert_eq!(
            solar_to_lunar(date).unwrap_err(),
            LunarError::InvalidSolarDate {
                year: date.year,
                month: date.month,
                day: date.day,
            },
            "{label} should be an invalid solar date"
        );
    }
}

#[test]
fn solar_to_lunar_accepts_gregorian_leap_day() {
    // 2024 is a Gregorian leap year, so Feb 29 is a real date and must not be
    // rejected as InvalidSolarDate.
    let result = solar_to_lunar(solar(2024, 2, 29));
    assert!(
        !matches!(result, Err(LunarError::InvalidSolarDate { .. })),
        "2024-02-29 is a valid Gregorian date, got {result:?}"
    );
    assert!(result.is_ok());
}

// --- Supported-range boundaries --------------------------------------------

#[test]
fn solar_to_lunar_first_supported_lunar_new_year() {
    // Generated data: lunar year 1850 new year is 1850-02-12.
    assert_eq!(
        solar_to_lunar(solar(1850, 2, 12)).unwrap(),
        lunar(1850, 1, 1, false)
    );
}

#[test]
fn solar_to_lunar_day_before_old_generated_range_new_year_is_supported() {
    assert_eq!(
        solar_to_lunar(solar(1850, 2, 11)).unwrap(),
        lunar(1849, 12, 30, false)
    );
}

#[test]
fn solar_to_lunar_near_upper_supported_range() {
    // Generated data: lunar year 2150 new year is 2150-01-29 (a leap year with
    // 13 months). 2150-12-31 sits comfortably inside that lunar year.
    assert_eq!(
        solar_to_lunar(solar(2150, 12, 31)).unwrap(),
        lunar(2150, 11, 13, false)
    );
}

#[test]
fn solar_to_lunar_lunar_2150_tail_in_gregorian_2151_is_supported() {
    assert_eq!(
        solar_to_lunar(solar(2151, 1, 18)).unwrap(),
        lunar(2150, 12, 1, false)
    );
}

// --- Chinese New Year round-trip anchors (solar -> lunar) -------------------

#[test]
fn solar_to_lunar_chinese_new_years() {
    let cases = [
        (solar(2023, 1, 22), lunar(2023, 1, 1, false)),
        (solar(2024, 2, 10), lunar(2024, 1, 1, false)),
        (solar(2025, 1, 29), lunar(2025, 1, 1, false)),
    ];

    for (input, expected) in cases {
        assert_eq!(solar_to_lunar(input).unwrap(), expected, "{input:?}");
    }
}

#[test]
fn solar_to_lunar_2023_leap_second_month() {
    assert_eq!(
        solar_to_lunar(solar(2023, 3, 22)).unwrap(),
        lunar(2023, 2, 1, true)
    );
}

// --- Year boundary around Chinese New Year ---------------------------------

#[test]
fn solar_to_lunar_year_boundary_around_2024_new_year() {
    // 2024-02-09 is the last day of lunar year 2023 (month 12, day 30);
    // 2024-02-10 is the first day of lunar year 2024.
    assert_eq!(
        solar_to_lunar(solar(2024, 2, 9)).unwrap(),
        lunar(2023, 12, 30, false)
    );
    assert_eq!(
        solar_to_lunar(solar(2024, 2, 10)).unwrap(),
        lunar(2024, 1, 1, false)
    );
}
