//! Use-level test that the Rust-native four-pillar API surface is public.
//!
//! Importing and calling these names is the assertion: if any were removed or
//! renamed, this test file would fail to compile. The JS-compatible aliases
//! were removed, so only the Rust-native names below must resolve.

use lunar_lite::{
    FourPillars, MonthDivide, SolarDate, StemBranchOptions, YearDivide,
    four_pillars_from_solar_date, four_pillars_from_solar_date_with_options,
};

#[test]
fn rust_native_four_pillar_names_are_available() {
    let solar = SolarDate {
        year: 2000,
        month: 8,
        day: 16,
    };

    let default: FourPillars = four_pillars_from_solar_date(solar, 2).unwrap();

    let options = StemBranchOptions {
        year: YearDivide::Exact,
        month: MonthDivide::Exact,
    };
    let explicit = four_pillars_from_solar_date_with_options(solar, 2, options).unwrap();

    // Default entry point equals explicit default (`Exact`/`Exact`) options.
    assert_eq!(default, explicit);
}
