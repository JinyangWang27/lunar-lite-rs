//! Julian Day utilities with tyme-compatible Julian/Gregorian reform handling.

use crate::SolarDate;

/// 2000-01-01 12:00:00 UTC.
pub(crate) const J2000: f64 = 2_451_545.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SolarDateTime {
    pub(crate) date: SolarDate,
    pub(crate) hour: u8,
    pub(crate) minute: u8,
    pub(crate) second: u8,
}

/// Returns the Julian Day for a calendar date and time.
///
/// Dates before 1582-10-15 are interpreted in the Julian calendar; dates on or
/// after 1582-10-15 are interpreted in the Gregorian calendar.
pub(crate) fn from_ymd_hms(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> f64 {
    let d = day as f64 + ((second as f64 / 60.0 + minute as f64) / 60.0 + hour as f64) / 24.0;
    let mut n = 0;
    let gregorian = year * 372 + month as i32 * 31 + d as i32 >= 588_829;
    let mut y = year;
    let mut m = month as i32;

    if m <= 2 {
        m += 12;
        y -= 1;
    }

    if gregorian {
        n = (y as f64 * 0.01) as i32;
        n = 2 - n + (n as f64 * 0.25) as i32;
    }

    (365.25 * (y + 4716) as f64) as i32 as f64
        + (30.6001 * (m + 1) as f64) as i32 as f64
        + d
        + n as f64
        - 1524.5
}

pub(crate) fn to_solar_datetime(julian_day: f64) -> SolarDateTime {
    let mut d = (julian_day + 0.5) as i32;
    let mut fraction = julian_day + 0.5 - d as f64;

    if d >= 2_299_161 {
        let c = ((d as f64 - 1_867_216.25) / 36_524.25) as i32;
        d += 1 + c - (c as f64 * 0.25) as i32;
    }

    d += 1524;
    let mut year = ((d as f64 - 122.1) / 365.25) as i32;
    d -= (365.25 * year as f64) as i32;
    let mut month = (d as f64 / 30.601) as i32;
    d -= (30.601 * month as f64) as i32;
    let day = d;

    if month > 13 {
        month -= 12;
    } else {
        year -= 1;
    }
    month -= 1;
    year -= 4715;

    fraction *= 24.0;
    let hour = fraction as u8;
    fraction -= hour as f64;
    fraction *= 60.0;
    let minute = fraction as u8;
    fraction -= minute as f64;
    fraction *= 60.0;
    let second = fraction.round() as u8;

    if second < 60 {
        return SolarDateTime {
            date: SolarDate {
                year,
                month: month as u8,
                day: day as u8,
            },
            hour,
            minute,
            second,
        };
    }

    add_seconds(SolarDateTime {
        date: SolarDate {
            year,
            month: month as u8,
            day: day as u8,
        },
        hour,
        minute,
        second: second - 60,
    })
}

pub(crate) fn to_solar_date(julian_day: f64) -> SolarDate {
    to_solar_datetime(julian_day).date
}

/// The sexagenary offset used for the day pillar: `floor(noonJulianDay) - 11`.
pub(crate) fn day_pillar_offset(year: i32, month: u8, day: u8) -> i64 {
    from_ymd_hms(year, month, day, 12, 0, 0).floor() as i64 - 11
}

fn add_seconds(time: SolarDateTime) -> SolarDateTime {
    let total_seconds = time.hour as u32 * 3600 + time.minute as u32 * 60 + time.second as u32 + 60;
    let day_offset = total_seconds / 86_400;
    let second_of_day = total_seconds % 86_400;
    let date = to_solar_date(from_ymd_hms(time.date.year, time.date.month, time.date.day, 0, 0, 0) + day_offset as f64);

    SolarDateTime {
        date,
        hour: (second_of_day / 3600) as u8,
        minute: (second_of_day % 3600 / 60) as u8,
        second: (second_of_day % 60) as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle(year: i32, month: u8, day: u8) -> (i64, i64) {
        let offset = day_pillar_offset(year, month, day);
        (offset.rem_euclid(10), offset.rem_euclid(12))
    }

    #[test]
    fn known_day_pillars_match_reference() {
        assert_eq!(cycle(1970, 1, 1), (7, 5));
        assert_eq!(cycle(2000, 1, 1), (4, 6));
        assert_eq!(cycle(1984, 2, 2), (2, 2));
        assert_eq!(cycle(1850, 1, 1), (8, 0));
        assert_eq!(cycle(2150, 12, 31), (5, 5));
    }

    #[test]
    fn julian_day_round_trips_across_reform() {
        let before = from_ymd_hms(1582, 10, 4, 0, 0, 0);
        let after = from_ymd_hms(1582, 10, 15, 0, 0, 0);

        assert_eq!(after - before, 1.0);
        assert_eq!(to_solar_date(before), SolarDate { year: 1582, month: 10, day: 4 });
        assert_eq!(to_solar_date(after), SolarDate { year: 1582, month: 10, day: 15 });
        assert_eq!(to_solar_date(before + 1.0), SolarDate { year: 1582, month: 10, day: 15 });
    }

    #[test]
    fn julian_calendar_leap_day_before_reform_round_trips() {
        let day = from_ymd_hms(1500, 2, 29, 0, 0, 0);
        assert_eq!(to_solar_date(day), SolarDate { year: 1500, month: 2, day: 29 });
    }
}
