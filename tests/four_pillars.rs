//! Fixture-driven compatibility tests for the four-pillar API.
//!
//! Fixtures in `fixtures/four_pillars.json` are generated from `lunar-lite@0.2.8`
//! (see `tools/lunar-lite-reference/scripts/generate-four-pillars-fixtures.mjs`).
#![cfg(feature = "serde")]

use lunar_lite::{
    FourPillars, SolarDate, StemBranchOptions, four_pillars_from_solar_date_with_options,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    solar: SolarDate,
    time_index: u8,
    options: StemBranchOptions,
    expected: FourPillars,
}

#[test]
fn fixtures_match_lunar_lite_reference() {
    let raw = include_str!("fixtures/four_pillars.json");
    let cases: Vec<Case> = serde_json::from_str(raw).expect("parse four_pillars.json");
    assert!(!cases.is_empty(), "no fixtures loaded");

    let mut failures = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        match four_pillars_from_solar_date_with_options(case.solar, case.time_index, case.options) {
            Ok(got) if got == case.expected => {}
            Ok(got) => failures.push(format!(
                "case {i}: {:?} ti={} {:?}\n  expected {:?}\n  got      {:?}",
                case.solar, case.time_index, case.options, case.expected, got
            )),
            Err(e) => failures.push(format!(
                "case {i}: {:?} ti={} {:?} errored: {e}",
                case.solar, case.time_index, case.options
            )),
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} fixtures mismatched:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
