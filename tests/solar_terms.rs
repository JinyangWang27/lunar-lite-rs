use lunar_lite::{
    LunarError, SolarDate, SolarTerm, is_on_or_after_li_chun, li_chun_date, solar_term_date,
};

fn solar(year: i32, month: u8, day: u8) -> SolarDate {
    SolarDate { year, month, day }
}

#[test]
fn solar_term_date_returns_known_li_chun_date() {
    assert_eq!(
        solar_term_date(2000, SolarTerm::LiChun).unwrap(),
        solar(2000, 2, 4)
    );
}

#[test]
fn li_chun_date_matches_solar_term_date_li_chun() {
    assert_eq!(
        li_chun_date(2024).unwrap(),
        solar_term_date(2024, SolarTerm::LiChun).unwrap()
    );
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
fn solar_term_date_rejects_year_before_supported_range() {
    assert_eq!(
        solar_term_date(1849, SolarTerm::LiChun),
        Err(LunarError::SolarTermOutOfRange { year: 1849 })
    );
}

#[test]
fn solar_term_date_rejects_year_after_supported_range() {
    assert_eq!(
        solar_term_date(2151, SolarTerm::LiChun),
        Err(LunarError::SolarTermOutOfRange { year: 2151 })
    );
}

#[test]
fn all_solar_terms_return_valid_dates_for_supported_year() {
    for term in [
        SolarTerm::LiChun,
        SolarTerm::YuShui,
        SolarTerm::JingZhe,
        SolarTerm::ChunFen,
        SolarTerm::QingMing,
        SolarTerm::GuYu,
        SolarTerm::LiXia,
        SolarTerm::XiaoMan,
        SolarTerm::MangZhong,
        SolarTerm::XiaZhi,
        SolarTerm::XiaoShu,
        SolarTerm::DaShu,
        SolarTerm::LiQiu,
        SolarTerm::ChuShu,
        SolarTerm::BaiLu,
        SolarTerm::QiuFen,
        SolarTerm::HanLu,
        SolarTerm::ShuangJiang,
        SolarTerm::LiDong,
        SolarTerm::XiaoXue,
        SolarTerm::DaXue,
        SolarTerm::DongZhi,
        SolarTerm::XiaoHan,
        SolarTerm::DaHan,
    ] {
        let date = solar_term_date(2024, term).unwrap();
        assert_eq!(date.year, 2024);
        assert!((1..=12).contains(&date.month));
        assert!((1..=31).contains(&date.day));
    }
}
