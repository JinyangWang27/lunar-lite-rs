use crate::astronomical::kernel::AstronomicalKernel;
use crate::julian_day::{J2000, SolarDateTime, to_solar_datetime};

const SOLAR_YEAR_DAYS: f64 = 365.2422;
const TERM_STEP_DAYS: f64 = 15.2184;
const WINTER_SOLSTICE_2000_OFFSET: f64 = 355.0;

/// Returns the cursory Julian-day offset from J2000 for a solar term.
///
/// `index` follows Tyme's 24-term order: 0 = winter solstice, 1 = minor cold,
/// 3 = start of spring, 23 = major snow.
pub(crate) fn term_cursory_offset(year: i32, index: i32) -> f64 {
    let y = (year * 24 + index).div_euclid(24);
    let term_index = (year * 24 + index).rem_euclid(24);
    let jd = ((y as f64 - 2000.0) * SOLAR_YEAR_DAYS + 180.0).floor();
    let mut winter_solstice =
        ((jd - WINTER_SOLSTICE_2000_OFFSET + 183.0) / SOLAR_YEAR_DAYS).floor() * SOLAR_YEAR_DAYS
            + WINTER_SOLSTICE_2000_OFFSET;

    if AstronomicalKernel::solar_term_day_offset(winter_solstice) > jd {
        winter_solstice -= SOLAR_YEAR_DAYS;
    }

    AstronomicalKernel::solar_term_day_offset(winter_solstice + TERM_STEP_DAYS * term_index as f64)
}

pub(crate) fn term_julian_day(year: i32, index: i32) -> f64 {
    AstronomicalKernel::refine_solar_term_offset(term_cursory_offset(year, index)) + J2000
}

pub(crate) fn term_datetime(year: i32, index: i32) -> SolarDateTime {
    to_solar_datetime(term_julian_day(year, index))
}

pub(crate) fn winter_solstice_cursory_offset(year: i32) -> f64 {
    term_cursory_offset(year, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd_hms(time: SolarDateTime) -> (i32, u8, u8, u8, u8, u8) {
        (
            time.date.year,
            time.date.month,
            time.date.day,
            time.hour,
            time.minute,
            time.second,
        )
    }

    #[test]
    fn known_li_chun_instants_match_tyme() {
        assert_eq!(ymd_hms(term_datetime(2000, 3)), (2000, 2, 4, 20, 40, 24));
        assert_eq!(ymd_hms(term_datetime(2024, 3)), (2024, 2, 4, 16, 27, 7));
        assert_eq!(ymd_hms(term_datetime(2034, 3)), (2034, 2, 4, 2, 41, 10));
    }
}
