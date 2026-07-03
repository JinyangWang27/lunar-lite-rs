//! Pre-Gregorian-reform (Julian calendar) behaviour for dates before
//! 1582-10-15. 1500 is a Julian leap year, so 1500-02-29 is a real day.

use lunar_lite::{
    MonthDivide, SolarDate, StemBranchOptions, YearDivide, four_pillars_from_solar_date,
    four_pillars_from_solar_date_with_options, solar_to_lunar,
};

fn solar(year: i32, month: u8, day: u8) -> SolarDate {
    SolarDate { year, month, day }
}

#[test]
fn julian_leap_day_validates_under_new_policy() {
    // All three dates are valid Julian-calendar dates and convert cleanly.
    for date in [solar(1500, 2, 28), solar(1500, 2, 29), solar(1500, 3, 1)] {
        assert!(solar_to_lunar(date).is_ok(), "{date:?} should convert");
    }
}

#[test]
fn four_pillar_exact_does_not_collapse_julian_leap_day() {
    let opts = StemBranchOptions {
        year: YearDivide::Exact,
        month: MonthDivide::Exact,
    };

    // Three consecutive Julian days must yield three distinct day pillars; if
    // 1500-02-29 collapsed onto 1500-03-01 two of them would coincide.
    let feb28 = four_pillars_from_solar_date_with_options(solar(1500, 2, 28), 6, opts).unwrap();
    let feb29 = four_pillars_from_solar_date_with_options(solar(1500, 2, 29), 6, opts).unwrap();
    let mar01 = four_pillars_from_solar_date_with_options(solar(1500, 3, 1), 6, opts).unwrap();

    assert_ne!(feb28.daily, feb29.daily);
    assert_ne!(feb29.daily, mar01.daily);
    assert_ne!(feb28.daily, mar01.daily);
}

#[test]
fn julian_leap_day_default_options_compute() {
    // The default-options entry point also handles the pre-reform leap day.
    assert!(four_pillars_from_solar_date(solar(1500, 2, 29), 6).is_ok());
}
