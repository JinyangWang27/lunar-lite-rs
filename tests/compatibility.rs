use lunar_lite::{
    LunarDate, SolarDate, lunar_to_solar, normalize_lunar_date, solar_to_lunar, time_index,
};

#[test]
fn solar_to_lunar_chinese_new_year_2023() {
    let lunar = solar_to_lunar(SolarDate {
        year: 2023,
        month: 1,
        day: 22,
    })
    .unwrap();

    assert_eq!(
        lunar,
        LunarDate {
            year: 2023,
            month: 1,
            day: 1,
            is_leap_month: false,
        }
    );
}

#[test]
fn solar_to_lunar_leap_month_2023() {
    let lunar = solar_to_lunar(SolarDate {
        year: 2023,
        month: 3,
        day: 22,
    })
    .unwrap();

    assert_eq!(
        lunar,
        LunarDate {
            year: 2023,
            month: 2,
            day: 1,
            is_leap_month: true,
        }
    );
}

#[test]
fn lunar_to_solar_leap_month_2023() {
    let solar = lunar_to_solar(LunarDate {
        year: 2023,
        month: 2,
        day: 1,
        is_leap_month: true,
    })
    .unwrap();

    assert_eq!(
        solar,
        SolarDate {
            year: 2023,
            month: 3,
            day: 22,
        }
    );
}

#[test]
fn normalize_fake_leap_month() {
    let normalized = normalize_lunar_date(LunarDate {
        year: 2024,
        month: 1,
        day: 1,
        is_leap_month: true,
    })
    .unwrap();

    assert_eq!(
        normalized,
        LunarDate {
            year: 2024,
            month: 1,
            day: 1,
            is_leap_month: false,
        }
    );
}

#[test]
fn time_index_distinguishes_early_and_late_zi() {
    assert_eq!(time_index(0, 0).unwrap(), 0);
    assert_eq!(time_index(0, 59).unwrap(), 0);
    assert_eq!(time_index(23, 0).unwrap(), 12);
    assert_eq!(time_index(23, 59).unwrap(), 12);
}
