//! Compatibility spike: can `tyme4rs` reproduce `lunar-lite` public behavior?
//!
//! These tests compare the current `lunar-lite` API against `tyme4rs` as an
//! external reference. A failure here is useful migration signal: it means the
//! behavior is not a drop-in match and needs an adapter, a policy switch, or a
//! documented incompatibility.

use lunar_lite as ll;
use tyme4rs::tyme::lunar::LunarDay as TymeLunarDay;
use tyme4rs::tyme::lunar::LunarMonth as TymeLunarMonth;
use tyme4rs::tyme::lunar::LunarYear as TymeLunarYear;
use tyme4rs::tyme::sixtycycle::SixtyCycle as TymeSixtyCycle;
use tyme4rs::tyme::solar::SolarDay as TymeSolarDay;
use tyme4rs::tyme::solar::SolarTime as TymeSolarTime;

fn solar(year: i32, month: u8, day: u8) -> ll::SolarDate {
    ll::SolarDate { year, month, day }
}

fn lunar(year: i32, month: u8, day: u8, is_leap_month: bool) -> ll::LunarDate {
    ll::LunarDate {
        year,
        month,
        day,
        is_leap_month,
    }
}

fn tyme_solar_day(date: ll::SolarDate) -> TymeSolarDay {
    TymeSolarDay::from_ymd(date.year as isize, date.month as usize, date.day as usize)
}

fn tyme_lunar_month_number(month: u8, is_leap_month: bool) -> isize {
    if is_leap_month {
        -(month as isize)
    } else {
        month as isize
    }
}

fn lunar_from_tyme(day: TymeLunarDay) -> ll::LunarDate {
    let month = day.get_month();
    ll::LunarDate {
        year: day.get_year() as i32,
        month: month.unsigned_abs() as u8,
        day: day.get_day() as u8,
        is_leap_month: month < 0,
    }
}

fn solar_from_tyme(day: TymeSolarDay) -> ll::SolarDate {
    ll::SolarDate {
        year: day.get_year() as i32,
        month: day.get_month() as u8,
        day: day.get_day() as u8,
    }
}

fn tyme_lunar_from_solar(date: ll::SolarDate) -> ll::LunarDate {
    lunar_from_tyme(tyme_solar_day(date).get_lunar_day())
}

fn tyme_solar_from_lunar(date: ll::LunarDate) -> ll::SolarDate {
    let tyme_lunar = TymeLunarDay::from_ymd(
        date.year as isize,
        tyme_lunar_month_number(date.month, date.is_leap_month),
        date.day as usize,
    );
    solar_from_tyme(tyme_lunar.get_solar_day())
}

fn stem_branch_from_tyme(value: TymeSixtyCycle) -> ll::StemBranch {
    ll::StemBranch::try_new(
        ll::HeavenlyStem::from_index(value.get_heaven_stem().get_index()),
        ll::EarthlyBranch::from_index(value.get_earth_branch().get_index()),
    )
    .expect("tyme4rs SixtyCycle should always be a valid stem-branch pair")
}

fn tyme_four_pillars(date: ll::SolarDate, time_idx: u8) -> ll::FourPillars {
    let hour = (time_idx as usize * 2).saturating_sub(1);
    let time = TymeSolarTime::from_ymd_hms(
        date.year as isize,
        date.month as usize,
        date.day as usize,
        hour,
        30,
        0,
    );
    let eight_char = time.get_lunar_hour().get_eight_char();

    ll::FourPillars {
        yearly: stem_branch_from_tyme(eight_char.get_year()),
        monthly: stem_branch_from_tyme(eight_char.get_month()),
        daily: stem_branch_from_tyme(eight_char.get_day()),
        hourly: stem_branch_from_tyme(eight_char.get_hour()),
    }
}

#[test]
fn solar_to_lunar_matches_tyme4rs_for_sampled_supported_dates() {
    let dates = [
        solar(1850, 2, 12),
        solar(1900, 1, 31),
        solar(1969, 7, 20),
        solar(1984, 2, 2),
        solar(2000, 2, 5),
        solar(2020, 5, 23),
        solar(2024, 2, 10),
        solar(2033, 12, 22),
        solar(2100, 1, 1),
        solar(2150, 12, 31),
    ];

    for date in dates {
        assert_eq!(ll::solar_to_lunar(date).unwrap(), tyme_lunar_from_solar(date));
    }
}

#[test]
fn lunar_to_solar_matches_tyme4rs_for_representative_dates() {
    let dates = [
        lunar(1850, 1, 1, false),
        lunar(1900, 1, 1, false),
        lunar(1984, 1, 1, false),
        lunar(2000, 1, 1, false),
        lunar(2020, 4, 1, true),
        lunar(2020, 4, 29, true),
        lunar(2024, 1, 1, false),
        lunar(2100, 1, 1, false),
        lunar(2150, 12, 29, false),
    ];

    for date in dates {
        assert_eq!(ll::lunar_to_solar(date).unwrap(), tyme_solar_from_lunar(date));
    }
}

#[test]
fn leap_month_and_month_lengths_match_tyme4rs_across_supported_years() {
    for year in 1850..=2150 {
        let tyme_leap_month = TymeLunarYear::from_year(year as isize).get_leap_month();
        let leap = ll::leap_month(year).unwrap();
        assert_eq!(
            leap,
            (tyme_leap_month > 0).then_some(tyme_leap_month as u8)
        );
        assert_eq!(ll::has_leap_month(year).unwrap(), tyme_leap_month > 0);

        for month in 1..=12 {
            let tyme_days = TymeLunarMonth::from_ym(year as isize, month as isize).get_day_count();
            assert_eq!(ll::lunar_month_days(year, month, false).unwrap(), tyme_days as u8);

            if tyme_leap_month == month as usize {
                let tyme_leap_days =
                    TymeLunarMonth::from_ym(year as isize, -(month as isize)).get_day_count();
                assert_eq!(
                    ll::lunar_month_days(year, month, true).unwrap(),
                    tyme_leap_days as u8
                );
            }
        }
    }
}

#[test]
fn strict_lunar_validation_matches_tyme4rs_validity_for_representative_dates() {
    let valid_dates = [
        lunar(2024, 1, 1, false),
        lunar(2020, 4, 1, true),
        lunar(2020, 4, 29, true),
        lunar(2033, 11, 1, false),
    ];

    for date in valid_dates {
        assert_eq!(ll::validate_lunar_date(date), Ok(()));
        assert_eq!(ll::lunar_to_solar(date).unwrap(), tyme_solar_from_lunar(date));
    }

    let fake_leap = lunar(2024, 1, 1, true);
    let normalized = ll::normalize_lunar_date(fake_leap).unwrap();
    assert_eq!(normalized, lunar(2024, 1, 1, false));
    assert_eq!(
        ll::lunar_to_solar(fake_leap).unwrap(),
        tyme_solar_from_lunar(normalized)
    );
}

#[test]
fn lunar_year_stem_branch_helpers_match_tyme4rs() {
    for year in [1850, 1900, 1984, 2000, 2024, 2033, 2100, 2150] {
        let tyme_cycle = TymeLunarYear::from_year(year as isize).get_sixty_cycle();
        let expected = stem_branch_from_tyme(tyme_cycle);
        assert_eq!(ll::lunar_year_stem_branch(year), expected);
        assert_eq!(ll::lunar_year_stem(year), expected.stem());
        assert_eq!(ll::lunar_year_branch(year), expected.branch());
    }
}

#[test]
fn time_index_semantics_match_tyme4rs_lunar_hour_index() {
    for (hour, expected_index) in [
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (22, 11),
        (23, 12),
    ] {
        assert_eq!(ll::time_index(hour, 30).unwrap(), expected_index);
        let tyme_index = TymeSolarTime::from_ymd_hms(2024, 2, 10, hour as usize, 30, 0)
            .get_lunar_hour()
            .get_index_in_day() as u8;
        assert_eq!(expected_index, tyme_index);
        assert_eq!(
            ll::time_index_to_branch(expected_index).unwrap().index(),
            (tyme_index % 12) as usize
        );
    }
}

#[test]
fn default_exact_four_pillars_match_tyme4rs_for_sampled_dates_and_time_indices() {
    let cases = [
        (solar(2000, 8, 16), 2),
        (solar(2024, 2, 3), 0),
        (solar(2024, 2, 4), 6),
        (solar(2024, 2, 10), 5),
        (solar(2024, 2, 10), 12),
        (solar(2033, 12, 22), 11),
    ];

    for (date, time_idx) in cases {
        let lunar_lite = ll::four_pillars_from_solar_date(date, time_idx).unwrap();
        let tyme = tyme_four_pillars(date, time_idx);
        assert_eq!(lunar_lite, tyme, "date={date:?}, time_index={time_idx}");
    }
}

#[test]
fn explicit_default_options_match_alias_and_tyme4rs() {
    let date = solar(2024, 2, 10);
    let time_idx = 5;
    let alias = ll::four_pillars_from_solar_date(date, time_idx).unwrap();
    let explicit = ll::four_pillars_from_solar_date_with_options(
        date,
        time_idx,
        ll::StemBranchOptions::default(),
    )
    .unwrap();

    assert_eq!(alias, explicit);
    assert_eq!(explicit, tyme_four_pillars(date, time_idx));
}
