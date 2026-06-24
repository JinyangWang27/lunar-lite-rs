use lunar_lite::{LunarError, SolarDate, is_on_or_after_li_chun, li_chun_date};

fn solar(year: i32, month: u8, day: u8) -> SolarDate {
    SolarDate { year, month, day }
}

#[test]
fn li_chun_date_returns_known_supported_date() {
    assert_eq!(li_chun_date(2000).unwrap(), solar(2000, 2, 4));
    assert_eq!(li_chun_date(2024).unwrap(), solar(2024, 2, 4));
}

#[test]
fn is_on_or_after_li_chun_is_false_before_li_chun() {
    assert!(!is_on_or_after_li_chun(solar(2024, 2, 3)).unwrap());
}

#[test]
fn is_on_or_after_li_chun_is_true_on_li_chun() {
    assert!(is_on_or_after_li_chun(solar(2024, 2, 4)).unwrap());
}

#[test]
fn is_on_or_after_li_chun_is_true_after_li_chun() {
    assert!(is_on_or_after_li_chun(solar(2024, 2, 5)).unwrap());
}

#[test]
fn li_chun_date_rejects_year_before_supported_range() {
    assert_eq!(
        li_chun_date(1849),
        Err(LunarError::SolarTermOutOfRange { year: 1849 })
    );
}

#[test]
fn li_chun_date_rejects_year_after_supported_range() {
    assert_eq!(
        li_chun_date(2151),
        Err(LunarError::SolarTermOutOfRange { year: 2151 })
    );
}

#[test]
fn is_on_or_after_li_chun_rejects_invalid_solar_date() {
    assert_eq!(
        is_on_or_after_li_chun(solar(2024, 2, 31)),
        Err(LunarError::InvalidSolarDate {
            year: 2024,
            month: 2,
            day: 31,
        })
    );
}
