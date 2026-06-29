use lunar_lite::{LunarError, SolarDate, SolarTermDateTime, li_chun_datetime};

#[test]
fn li_chun_2000_exact_datetime() {
    let dt = li_chun_datetime(2000).unwrap();
    assert_eq!(
        dt,
        SolarTermDateTime {
            date: SolarDate {
                year: 2000,
                month: 2,
                day: 4
            },
            hour: 20,
            minute: 40,
            second: 24,
        }
    );
}

#[test]
fn li_chun_2024_known_values() {
    let dt = li_chun_datetime(2024).unwrap();
    assert_eq!(dt.date.year, 2024);
    assert_eq!(dt.date.month, 2);
    assert_eq!(dt.date.day, 4);
    // Exact time cross-checked against the internal test in astronomical/solar_term.rs.
    assert_eq!(dt.hour, 16);
    assert_eq!(dt.minute, 27);
    assert_eq!(dt.second, 7);
}

#[test]
fn li_chun_year_zero_is_out_of_range() {
    assert_eq!(
        li_chun_datetime(0).unwrap_err(),
        LunarError::SolarTermOutOfRange { year: 0 },
    );
}

#[test]
fn li_chun_year_10000_is_out_of_range() {
    assert_eq!(
        li_chun_datetime(10_000).unwrap_err(),
        LunarError::SolarTermOutOfRange { year: 10_000 },
    );
}

#[cfg(feature = "serde")]
mod serde_tests {
    use lunar_lite::{SolarDate, SolarTermDateTime};

    #[test]
    fn solar_term_datetime_round_trips_through_json() {
        let dt = SolarTermDateTime {
            date: SolarDate {
                year: 2000,
                month: 2,
                day: 4,
            },
            hour: 20,
            minute: 40,
            second: 24,
        };
        let json = serde_json::to_string(&dt).unwrap();
        let back: SolarTermDateTime = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, back);
    }
}
