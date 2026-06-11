use lunar_lite::{LunarDate, LunarError, normalize_lunar_date};

#[test]
fn normalize_fake_leap_month_with_valid_day() {
    let normalized = normalize_lunar_date(LunarDate {
        year: 2024,
        month: 1,
        day: 29,
        is_leap_month: true,
    })
    .unwrap();

    assert_eq!(
        normalized,
        LunarDate {
            year: 2024,
            month: 1,
            day: 29,
            is_leap_month: false,
        }
    );
}

#[test]
fn normalize_fake_leap_month_then_reject_invalid_day() {
    let err = normalize_lunar_date(LunarDate {
        year: 2024,
        month: 1,
        day: 30,
        is_leap_month: true,
    })
    .unwrap_err();

    assert_eq!(
        err,
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 1,
            day: 30,
            is_leap_month: false,
        }
    );
}

#[test]
fn normalize_regular_month_rejects_invalid_day() {
    let err = normalize_lunar_date(LunarDate {
        year: 2023,
        month: 1,
        day: 30,
        is_leap_month: false,
    })
    .unwrap_err();

    assert_eq!(
        err,
        LunarError::InvalidLunarDate {
            year: 2023,
            month: 1,
            day: 30,
            is_leap_month: false,
        }
    );
}

#[test]
fn normalize_real_leap_month_with_valid_day() {
    let normalized = normalize_lunar_date(LunarDate {
        year: 2023,
        month: 2,
        day: 29,
        is_leap_month: true,
    })
    .unwrap();

    assert_eq!(
        normalized,
        LunarDate {
            year: 2023,
            month: 2,
            day: 29,
            is_leap_month: true,
        }
    );
}

#[test]
fn normalize_real_leap_month_rejects_invalid_day() {
    let err = normalize_lunar_date(LunarDate {
        year: 2023,
        month: 2,
        day: 30,
        is_leap_month: true,
    })
    .unwrap_err();

    assert_eq!(
        err,
        LunarError::InvalidLunarDate {
            year: 2023,
            month: 2,
            day: 30,
            is_leap_month: true,
        }
    );
}

#[test]
fn normalize_rejects_invalid_basic_shape() {
    let err = normalize_lunar_date(LunarDate {
        year: 2024,
        month: 13,
        day: 1,
        is_leap_month: false,
    })
    .unwrap_err();

    assert_eq!(
        err,
        LunarError::InvalidLunarDate {
            year: 2024,
            month: 13,
            day: 1,
            is_leap_month: false,
        }
    );
}
