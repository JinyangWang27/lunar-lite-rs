//! Julian Day arithmetic for the day pillar.
//!
//! `lunar-typescript` derives the day Ganzhi from the Julian Day Number at noon:
//! `offset = floor(julianDay) - 11`, then `offset % 10` / `offset % 12`. For the
//! supported range (1850..=2150, all after the 1582 Gregorian reform) the noon
//! Julian Day Number equals `days_from_civil(y, m, d) + 2440588`, so we reuse the
//! existing proleptic-Gregorian day count instead of recomputing the astronomical
//! formula.

use crate::calendar::days_from_civil;

/// Julian Day Number at 1970-01-01 12:00 UTC (`floor(2440587.5 + 0.5)`).
const NOON_JDN_AT_EPOCH: i64 = 2_440_588;

/// The sexagenary offset used for the day pillar: `floor(noonJulianDay) - 11`.
///
/// The day pillar's cycle index is `day_pillar_offset(...).rem_euclid(60)`.
pub(crate) fn day_pillar_offset(year: i32, month: u8, day: u8) -> i64 {
    days_from_civil(year, month, day) as i64 + NOON_JDN_AT_EPOCH - 11
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
        // Values from lunar-typescript@1.8.6 (stem index, branch index).
        // 1970-01-01 -> 辛巳 (Xin=7, Si=5)
        assert_eq!(cycle(1970, 1, 1), (7, 5));
        // 2000-01-01 -> 戊午 (Wu=4, Wu=6)
        assert_eq!(cycle(2000, 1, 1), (4, 6));
        // 1984-02-02 -> 丙寅 (Bing=2, Yin=2)
        assert_eq!(cycle(1984, 2, 2), (2, 2));
        // 1850-01-01 -> 壬子 (Ren=8, Zi=0)
        assert_eq!(cycle(1850, 1, 1), (8, 0));
        // 2150-12-31 -> 己巳 (Ji=5, Si=5)
        assert_eq!(cycle(2150, 12, 31), (5, 5));
    }

    #[test]
    fn noon_jdn_matches_reference_floor() {
        // floor(julianDay) at noon, straight from the reference.
        assert_eq!(day_pillar_offset(1970, 1, 1) + 11, 2_440_588);
        assert_eq!(day_pillar_offset(2000, 1, 1) + 11, 2_451_545);
        assert_eq!(day_pillar_offset(1850, 1, 1) + 11, 2_396_759);
        assert_eq!(day_pillar_offset(2150, 12, 31) + 11, 2_506_696);
    }
}
