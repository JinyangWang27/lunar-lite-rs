//! Self-standing invariant tests.
//!
//! Unlike the `tyme_compatibility` / `tyme_oracle_compatibility` suites, these
//! tests assert properties that must hold for `lunar-lite` on its own terms —
//! round-trip identity, calendar-shape bounds, boundary monotonicity, and the
//! documented late-子时 rule. They stay meaningful even if the `tyme4rs`
//! dev-dependency is bumped or removed.

use lunar_lite::{
    EarthlyBranch, LunarDate, SolarDate, StemBranch, four_pillars_from_solar_date, leap_month,
    li_chun_datetime, lunar_month_days, lunar_to_solar, lunar_year_stem_branch,
    normalize_lunar_date, solar_to_lunar,
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

// --- 1. Solar round trip ----------------------------------------------------

/// `lunar_to_solar(solar_to_lunar(s)) == s` for every valid solar date `s`.
#[test]
fn solar_round_trip_holds_across_range() {
    // Interior dates avoid the year-1 lower and year-9999 upper solar limits
    // while still exercising the required years.
    let years = [1, 18, 1500, 1582, 1900, 2000, 2024, 2100, 9999];
    for year in years {
        for (month, day) in [(3u8, 15u8), (6, 15), (9, 15)] {
            let s = solar(year, month, day);
            let l = solar_to_lunar(s).unwrap_or_else(|e| panic!("solar_to_lunar {s:?}: {e:?}"));
            let back = lunar_to_solar(l).unwrap_or_else(|e| panic!("lunar_to_solar {l:?}: {e:?}"));
            assert_eq!(back, s, "solar round trip failed for {s:?} (via {l:?})");
        }
    }
}

/// The Gregorian reform boundary round-trips on both valid edges, and every day
/// in the gap `1582-10-05..=1582-10-14` is rejected.
#[test]
fn solar_round_trip_reform_boundary() {
    for s in [solar(1582, 10, 4), solar(1582, 10, 15)] {
        let l = solar_to_lunar(s).unwrap();
        assert_eq!(lunar_to_solar(l).unwrap(), s, "reform edge {s:?}");
    }

    for day in 5..=14u8 {
        assert!(
            solar_to_lunar(solar(1582, 10, day)).is_err(),
            "reform gap 1582-10-{day} must be invalid"
        );
    }
}

// --- 2. Lunar round trip ----------------------------------------------------

/// `solar_to_lunar(lunar_to_solar(l)) == normalize_lunar_date(l)`.
///
/// The right-hand side normalizes fake leap flags, so this holds for regular
/// months, real leap months, and dropped fake-leap inputs alike.
#[test]
fn lunar_round_trip_equals_normalized_input() {
    let cases = [
        lunar(2024, 3, 15, false), // regular month
        lunar(2023, 2, 15, false), // regular month sharing a number with a leap
        lunar(2023, 2, 15, true),  // real leap month (2023 leaps month 2)
        lunar(2020, 4, 29, true),  // real leap month, last day
        lunar(2024, 1, 1, true),   // fake leap flag: 2024 has no leap month 1
        lunar(1900, 8, 1, false),  // early modern anchor
    ];

    for l in cases {
        let s = lunar_to_solar(l).unwrap_or_else(|e| panic!("lunar_to_solar {l:?}: {e:?}"));
        let back = solar_to_lunar(s).unwrap_or_else(|e| panic!("solar_to_lunar {s:?}: {e:?}"));
        let expected = normalize_lunar_date(l).unwrap();
        assert_eq!(
            back, expected,
            "lunar round trip failed for {l:?} (via {s:?})"
        );
    }
}

// --- 3. Month / year shape invariants ---------------------------------------

/// Every valid lunar month is 29 or 30 days long, regular or leap.
#[test]
fn lunar_month_days_are_29_or_30() {
    for year in [1, 1500, 1900, 1984, 2000, 2020, 2023, 2024, 2100, 9998] {
        for month in 1..=12u8 {
            let days = lunar_month_days(year, month, false).unwrap();
            assert!(
                days == 29 || days == 30,
                "regular {year}-{month} has {days} days"
            );

            if leap_month(year).unwrap() == Some(month) {
                let leap_days = lunar_month_days(year, month, true).unwrap();
                assert!(
                    leap_days == 29 || leap_days == 30,
                    "leap {year}-{month} has {leap_days} days"
                );
            }
        }
    }
}

/// A lunar year's total length is 353..=355 days when common (12 months) and
/// 383..=385 when it has a leap month (13 months).
#[test]
fn lunar_year_day_sums_are_in_range() {
    for year in [
        1, 1500, 1900, 1984, 2000, 2020, 2023, 2024, 2033, 2100, 9998,
    ] {
        let mut total: u32 = 0;
        for month in 1..=12u8 {
            total += lunar_month_days(year, month, false).unwrap() as u32;
        }

        let leap = leap_month(year).unwrap();
        if let Some(lm) = leap {
            total += lunar_month_days(year, lm, true).unwrap() as u32;
        }

        if leap.is_some() {
            assert!(
                (383..=385).contains(&total),
                "leap lunar year {year} has {total} days"
            );
        } else {
            assert!(
                (353..=355).contains(&total),
                "common lunar year {year} has {total} days"
            );
        }
    }
}

// --- 5. LiChun public/internal lockstep (behavioural) -----------------------

/// The `YearDivide::Exact` year pillar flips exactly at the public
/// `li_chun_datetime` date: on that day the pillar is the current suì-year
/// stem-branch, and the day before it is the previous year's.
#[test]
fn year_pillar_flips_at_public_li_chun_date() {
    for year in [1, 1582, 1900, 2000, 2024, 2100, 9999] {
        let li_chun = li_chun_datetime(year).unwrap().date;

        // On LiChun the year pillar is `year`'s sexagenary stem-branch.
        let on = four_pillars_from_solar_date(li_chun, 6).unwrap();
        assert_eq!(
            on.yearly,
            lunar_year_stem_branch(year),
            "year pillar on LiChun {li_chun:?}"
        );

        // The day before LiChun still belongs to the previous suì year.
        let before = solar(li_chun.year, li_chun.month, li_chun.day - 1);
        let prev = four_pillars_from_solar_date(before, 6).unwrap();
        assert_eq!(
            prev.yearly,
            lunar_year_stem_branch(year - 1),
            "year pillar day before LiChun {before:?}"
        );
    }
}

// --- 6. Late 子时 invariant --------------------------------------------------

/// `time_index` 12 (late 子) maps to the 子 branch and rolls the day pillar
/// one sexagenary step forward relative to `time_index` 0 (early 子) on the
/// same calendar date.
#[test]
fn late_zi_maps_to_zi_and_rolls_day_pillar() {
    for s in [solar(2000, 8, 16), solar(1984, 2, 2), solar(2024, 6, 1)] {
        let early = four_pillars_from_solar_date(s, 0).unwrap();
        let late = four_pillars_from_solar_date(s, 12).unwrap();

        assert_eq!(
            early.hourly.branch(),
            EarthlyBranch::Zi,
            "early 子 branch {s:?}"
        );
        assert_eq!(
            late.hourly.branch(),
            EarthlyBranch::Zi,
            "late 子 branch {s:?}"
        );

        let expected = StemBranch::from_cycle_index((early.daily.cycle_index() + 1) % 60);
        assert_eq!(
            late.daily, expected,
            "late 子 must roll the day pillar forward for {s:?}"
        );
    }
}
