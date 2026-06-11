use lunar_lite::{LunarDate, LunarError, SolarDate, lunar_to_solar};

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

// --- Chinese New Year round-trip anchors (lunar -> solar) -------------------

#[test]
fn lunar_to_solar_chinese_new_years() {
    let cases = [
        (lunar(2023, 1, 1, false), solar(2023, 1, 22)),
        (lunar(2024, 1, 1, false), solar(2024, 2, 10)),
        (lunar(2025, 1, 1, false), solar(2025, 1, 29)),
    ];

    for (input, expected) in cases {
        assert_eq!(lunar_to_solar(input).unwrap(), expected, "{input:?}");
    }
}

#[test]
fn lunar_to_solar_2023_leap_second_month() {
    assert_eq!(
        lunar_to_solar(lunar(2023, 2, 1, true)).unwrap(),
        solar(2023, 3, 22)
    );
}

// The lunar 2150 tail that `solar_to_lunar` cannot reach (see solar_to_lunar.rs)
// is still produced by `lunar_to_solar`, landing in Gregorian 2151. This is the
// other half of the boundary asymmetry regression.
#[test]
fn lunar_to_solar_lunar_2150_tail_lands_in_gregorian_2151() {
    assert_eq!(
        lunar_to_solar(lunar(2150, 12, 1, false)).unwrap(),
        solar(2151, 1, 18)
    );
}

// --- Invalid lunar dates ----------------------------------------------------

#[test]
fn lunar_to_solar_rejects_invalid_basic_shape() {
    let cases = [
        ("month 0", lunar(2024, 0, 1, false)),
        ("month 13", lunar(2024, 13, 1, false)),
        ("day 0", lunar(2024, 1, 0, false)),
        ("day 31", lunar(2024, 1, 31, false)),
    ];

    for (label, date) in cases {
        assert_eq!(
            lunar_to_solar(date).unwrap_err(),
            LunarError::InvalidLunarDate {
                year: date.year,
                month: date.month,
                day: date.day,
                is_leap_month: date.is_leap_month,
            },
            "{label} should be an invalid lunar date"
        );
    }
}

#[test]
fn lunar_to_solar_rejects_day_30_in_29_day_month() {
    // Generated data: lunar 2024 month 1 has 29 days.
    assert_eq!(
        lunar_to_solar(lunar(2024, 1, 30, false)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 1,
            day: 30,
            is_leap_month: false,
        }
    );
}

#[test]
fn lunar_to_solar_rejects_invalid_day_in_real_leap_month() {
    // Generated data: lunar 2023 leap month 2 has 29 days; the leap flag is
    // genuine and must be preserved in the error.
    assert_eq!(
        lunar_to_solar(lunar(2023, 2, 30, true)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2023,
            month: 2,
            day: 30,
            is_leap_month: true,
        }
    );
}

#[test]
fn lunar_to_solar_rejects_invalid_day_after_fake_leap_normalized_away() {
    // 2024 has no leap month, so the leap flag is normalized away before
    // validation; the resulting error reports is_leap_month: false.
    assert_eq!(
        lunar_to_solar(lunar(2024, 1, 30, true)).unwrap_err(),
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 1,
            day: 30,
            is_leap_month: false,
        }
    );
}

#[test]
fn lunar_to_solar_rejects_out_of_range_year() {
    assert_eq!(
        lunar_to_solar(lunar(1849, 1, 1, false)).unwrap_err(),
        LunarError::YearOutOfRange { year: 1849 }
    );
}
