//! Broad `tyme4rs` compatibility confidence tests.
//!
//! These tests treat `tyme4rs = "=1.5.0"` as an oracle and check that
//! `lunar-lite` agrees with it across a wide range of lunar-month facts and
//! conversions. They exist to build migration confidence for downstream crates
//! (such as `iztro-rs`) before they switch from `tyme4rs` back to `lunar-lite`.
//!
//! `tyme4rs` is a test-only `dev-dependency`; `lunar-lite` itself never depends
//! on it at runtime.

use lunar_lite::{
    EarthlyBranch, HeavenlyStem, LunarDate, SolarDate, StemBranch, leap_month, lunar_month_days,
    lunar_to_solar, solar_to_lunar,
};
use tyme4rs::tyme::lunar::{LunarDay, LunarMonth, LunarYear};
use tyme4rs::tyme::solar::SolarDay;

/// Signed tyme month: positive for a regular month, negative for a leap month.
fn tyme_signed_month(month: u8, is_leap_month: bool) -> isize {
    if is_leap_month {
        -(month as isize)
    } else {
        month as isize
    }
}

/// Asserts `lunar_to_solar` agrees with the tyme oracle for one lunar date.
fn assert_lunar_to_solar_matches_tyme(year: i32, month: u8, day: u8, is_leap_month: bool) {
    let lite = lunar_to_solar(LunarDate {
        year,
        month,
        day,
        is_leap_month,
    })
    .unwrap_or_else(|error| {
        panic!("lunar_to_solar failed for {year}-{month}-{day} leap={is_leap_month}: {error:?}")
    });

    let signed = tyme_signed_month(month, is_leap_month);
    let tyme = LunarDay::from_ymd(year as isize, signed, day as usize).get_solar_day();
    let expected = SolarDate {
        year: tyme.get_year() as i32,
        month: tyme.get_month() as u8,
        day: tyme.get_day() as u8,
    };

    assert_eq!(
        lite, expected,
        "lunar_to_solar mismatch for lunar {year}-{month}-{day} leap={is_leap_month} \
         (tyme expected {expected:?})"
    );
}

/// Asserts `solar_to_lunar` agrees with the tyme oracle for one solar date.
fn assert_solar_to_lunar_matches_tyme(year: i32, month: u8, day: u8) {
    let lite = solar_to_lunar(SolarDate { year, month, day }).unwrap_or_else(|error| {
        panic!("solar_to_lunar failed for {year}-{month}-{day}: {error:?}")
    });

    let tyme_day = SolarDay::from_ymd(year as isize, month as usize, day as usize).get_lunar_day();
    let tyme_month = tyme_day.get_lunar_month();
    let expected = LunarDate {
        year: tyme_month.get_year() as i32,
        month: tyme_month.get_month() as u8,
        day: tyme_day.get_day() as u8,
        is_leap_month: tyme_month.is_leap(),
    };

    assert_eq!(
        lite, expected,
        "solar_to_lunar mismatch for solar {year}-{month}-{day} (tyme expected {expected:?})"
    );
}

/// 1. Exhaustive lunar-year / lunar-month facts against tyme4rs.
///
/// Iterates every supported lunar year `-1..=9999` and compares leap-month
/// numbers and per-month day counts against tyme4rs.
///
/// tyme4rs derives a lunar year's month lengths from the *previous* lunar
/// year's head, so its day-count APIs require lunar year `>= 0` (lunar year
/// `-1` would force it to read year `-2`, outside its `-1..=9999` range and
/// panicking). The leap-month *number* for year `-1` is still a stable fact in
/// both libraries, so that comparison runs for the full range; the per-month
/// day-count comparison runs for `0..=9999`. lunar-lite's own leap-month error
/// behaviour is still exercised for year `-1`.
#[test]
fn lunar_month_facts_match_tyme_for_all_years() {
    for year in -1..=9999 {
        let lite_leap = leap_month(year).unwrap();
        let tyme_leap = LunarYear::from_year(year as isize).get_leap_month();

        match lite_leap {
            Some(month) => assert_eq!(
                month as usize, tyme_leap,
                "leap month mismatch for {year}: lite={month} tyme={tyme_leap}"
            ),
            None => assert_eq!(
                0, tyme_leap,
                "leap month mismatch for {year}: lite=None tyme={tyme_leap}"
            ),
        }

        for month in 1..=12u8 {
            // tyme cannot compute day counts for lunar year -1 (see fn doc).
            if year >= 0 {
                let lite_days = lunar_month_days(year, month, false).unwrap();
                let tyme_days = LunarMonth::from_ym(year as isize, month as isize).get_day_count();
                assert_eq!(
                    lite_days as usize, tyme_days,
                    "lunar month days mismatch for {year}-{month}"
                );
            }

            if lite_leap == Some(month) {
                if year >= 0 {
                    let lite_leap_days = lunar_month_days(year, month, true).unwrap();
                    let tyme_leap_days =
                        LunarMonth::from_ym(year as isize, -(month as isize)).get_day_count();
                    assert_eq!(
                        lite_leap_days as usize, tyme_leap_days,
                        "leap lunar month days mismatch for {year}-{month}"
                    );
                }
            } else {
                assert!(
                    lunar_month_days(year, month, true).is_err(),
                    "expected leap month error for non-leap {year}-{month}"
                );
            }
        }
    }
}

/// 2. Lunar-to-solar month-boundary samples against tyme4rs.
///
/// Compares the first and last day of every regular month (and every leap
/// month) against tyme4rs for lunar years `1..=9998`. Lunar year `-1` is
/// skipped because its dates land before solar year 1; the late `9999` tail is
/// skipped because some of its days land outside solar year 9999.
#[test]
fn lunar_to_solar_boundaries_match_tyme() {
    for year in 1..=9998 {
        let leap = leap_month(year).unwrap();

        for month in 1..=12u8 {
            let days = lunar_month_days(year, month, false).unwrap();
            assert_lunar_to_solar_matches_tyme(year, month, 1, false);
            assert_lunar_to_solar_matches_tyme(year, month, days, false);

            if leap == Some(month) {
                let leap_days = lunar_month_days(year, month, true).unwrap();
                assert_lunar_to_solar_matches_tyme(year, month, 1, true);
                assert_lunar_to_solar_matches_tyme(year, month, leap_days, true);
            }
        }
    }
}

/// 3. Sampled solar-to-lunar dates against tyme4rs.
///
/// Uses a deterministic sample: four spread-out dates for every year
/// `1..=9999`, dense day-1/15/28 coverage for `1900..=2100`, plus the Gregorian
/// reform boundary.
#[test]
fn solar_to_lunar_samples_match_tyme() {
    // Four spread-out valid dates per year across the whole supported range.
    //
    // Solar 18-12-28 is skipped as an oracle comparison: tyme4rs's early-year
    // new-moon offset (the `year > 8 && year < 24` branch) produces an
    // out-of-range lunar day there and panics. lunar-lite handles the date, so
    // it is still asserted to convert successfully below.
    for year in 1..=9999 {
        for (month, day) in [(1u8, 1u8), (2, 1), (6, 15), (12, 28)] {
            if (year, month, day) == (18, 12, 28) {
                continue;
            }
            assert_solar_to_lunar_matches_tyme(year, month, day);
        }
    }
    assert!(
        solar_to_lunar(SolarDate {
            year: 18,
            month: 12,
            day: 28,
        })
        .is_ok(),
        "lunar-lite should convert 18-12-28 even though tyme4rs panics on it"
    );

    // Dense modern coverage.
    for year in 1900..=2100 {
        for month in 1..=12u8 {
            for day in [1u8, 15, 28] {
                assert_solar_to_lunar_matches_tyme(year, month, day);
            }
        }
    }

    // Gregorian reform boundary: 1582-10-04 and 1582-10-15 are valid, the gap
    // 1582-10-05..=1582-10-14 is invalid in lunar-lite.
    assert_solar_to_lunar_matches_tyme(1582, 10, 4);
    assert_solar_to_lunar_matches_tyme(1582, 10, 15);
    for day in 5..=14u8 {
        assert!(
            solar_to_lunar(SolarDate {
                year: 1582,
                month: 10,
                day,
            })
            .is_err(),
            "expected reform-gap date 1582-10-{day} to be invalid"
        );
    }
}

/// 4. Public GanZhi types are usable directly by downstream crates.
///
/// A smoke test confirming the re-exported sexagenary types behave as expected
/// without reaching into `lunar-lite` internals.
#[test]
fn ganzhi_public_types_smoke_test() {
    let jia_zi = StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi)
        .expect("Jia-Zi is a valid stem-branch pair");

    assert_eq!(StemBranch::from_cycle_index(0), jia_zi);
    assert_eq!(jia_zi.stem(), HeavenlyStem::Jia);
    assert_eq!(jia_zi.branch(), EarthlyBranch::Zi);
    assert_eq!(jia_zi.cycle_index(), 0);

    assert_eq!(HeavenlyStem::from_index(0), HeavenlyStem::Jia);
    assert_eq!(EarthlyBranch::from_index(0), EarthlyBranch::Zi);

    // An incompatible stem/branch parity is rejected.
    assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chou).is_err());
}
