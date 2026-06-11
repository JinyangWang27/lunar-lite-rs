#![cfg(feature = "serde")]

use lunar_lite::{LunarDate, SolarDate};

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
