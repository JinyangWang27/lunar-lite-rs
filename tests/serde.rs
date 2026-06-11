#![cfg(feature = "serde")]

use lunar_lite::{LunarDate, SolarDate, StemBranch};

#[test]
fn solar_date_round_trips_through_json() {
    let date = SolarDate {
        year: 2024,
        month: 2,
        day: 10,
    };

    let json = serde_json::to_string(&date).unwrap();
    let back: SolarDate = serde_json::from_str(&json).unwrap();

    assert_eq!(date, back);
}

#[test]
fn lunar_date_round_trips_through_json() {
    let date = LunarDate {
        year: 2023,
        month: 2,
        day: 1,
        is_leap_month: true,
    };

    let json = serde_json::to_string(&date).unwrap();
    let back: LunarDate = serde_json::from_str(&json).unwrap();

    assert_eq!(date, back);
}

#[test]
fn serde_rejects_invalid_stem_branch_pair() {
    let json = r#"{"stem":"jia","branch":"chou"}"#;
    let err = serde_json::from_str::<StemBranch>(json).unwrap_err();
    assert!(err.to_string().contains("Invalid stem-branch pair"));
}
