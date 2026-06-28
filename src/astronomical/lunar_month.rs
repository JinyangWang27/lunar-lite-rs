use crate::astronomical::{lunar_year, solar_term};
use crate::julian_day::{J2000, to_solar_date};
use crate::{LunarError, SolarDate};

use super::shouxing::ShouXingUtil;

const SYNODIC_MONTH_DAYS: f64 = 29.5306;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LunarMonth {
    year: i32,
    month: u8,
    is_leap: bool,
}

impl LunarMonth {
    pub(crate) fn from_ym(year: i32, month_with_leap: i8) -> Result<Self, LunarError> {
        lunar_year::validate_lunar_year(year)?;

        if month_with_leap == 0 || !(-12..=12).contains(&month_with_leap) {
            return Err(invalid_lunar_month(
                year,
                month_with_leap.unsigned_abs(),
                month_with_leap < 0,
            ));
        }

        let month = month_with_leap.unsigned_abs();
        let is_leap = month_with_leap < 0;
        if is_leap && lunar_year::leap_month(year)? != Some(month) {
            return Err(invalid_lunar_month(year, month, true));
        }

        Ok(Self {
            year,
            month,
            is_leap,
        })
    }

    pub(crate) fn next(self, n: i32) -> Result<Self, LunarError> {
        if n == 0 {
            return Ok(self);
        }

        let mut index = self.index_in_year()? as i32 + 1 + n;
        let mut year = self.year;
        let mut month_count = lunar_year_month_count(year)?;
        let forward = n > 0;
        let add = if forward { 1 } else { -1 };

        while if forward {
            index > month_count
        } else {
            index <= 0
        } {
            if forward {
                index -= month_count;
            }
            year += add;
            lunar_year::validate_lunar_year(year)?;
            month_count = lunar_year_month_count(year)?;
            if !forward {
                index += month_count;
            }
        }

        let mut is_leap = false;
        let leap_month = lunar_year::leap_month(year)?.unwrap_or(0);
        if leap_month > 0 {
            if index == leap_month as i32 + 1 {
                is_leap = true;
            }
            if index > leap_month as i32 {
                index -= 1;
            }
        }

        Self::from_ym(year, if is_leap { -(index as i8) } else { index as i8 })
    }

    pub(crate) fn year(self) -> i32 {
        self.year
    }

    pub(crate) fn month(self) -> u8 {
        self.month
    }

    pub(crate) fn month_with_leap(self) -> i8 {
        if self.is_leap {
            -(self.month as i8)
        } else {
            self.month as i8
        }
    }

    pub(crate) fn is_leap(self) -> bool {
        self.is_leap
    }

    pub(crate) fn index_in_year(self) -> Result<u8, LunarError> {
        let mut index = self.month - 1;
        if self.is_leap {
            index += 1;
        } else if let Some(leap_month) = lunar_year::leap_month(self.year)? {
            if self.month > leap_month {
                index += 1;
            }
        }
        Ok(index)
    }

    pub(crate) fn first_julian_day(self) -> Result<f64, LunarError> {
        Ok(J2000 + ShouXingUtil::calc_shuo(self.new_moon_offset()?))
    }

    pub(crate) fn first_solar_date(self) -> Result<SolarDate, LunarError> {
        Ok(to_solar_date(self.first_julian_day()?))
    }

    pub(crate) fn day_count(self) -> Result<u8, LunarError> {
        let new_moon = self.new_moon_offset()?;
        Ok((ShouXingUtil::calc_shuo(new_moon + SYNODIC_MONTH_DAYS)
            - ShouXingUtil::calc_shuo(new_moon)) as u8)
    }

    fn new_moon_offset(self) -> Result<f64, LunarError> {
        let dong_zhi_jd = solar_term::winter_solstice_cursory_offset(self.year);
        let mut w = ShouXingUtil::calc_shuo(dong_zhi_jd);
        if w > dong_zhi_jd {
            w -= 29.53;
        }

        let previous_leap_month = lunar_year::leap_month(self.year - 1)?.unwrap_or(0);
        let mut offset = 2.0;
        if self.year > 8 && self.year < 24 {
            offset = 1.0;
        } else if previous_leap_month > 10 && self.year != 239 && self.year != 240 {
            offset = 3.0;
        }

        Ok(w + SYNODIC_MONTH_DAYS * (offset + self.index_in_year()? as f64))
    }
}

fn lunar_year_month_count(year: i32) -> Result<i32, LunarError> {
    Ok(if lunar_year::leap_month(year)?.is_some() {
        13
    } else {
        12
    })
}

fn invalid_lunar_month(year: i32, month: u8, is_leap_month: bool) -> LunarError {
    LunarError::InvalidLunarDate {
        year,
        month,
        day: 1,
        is_leap_month,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_lengths_and_first_days_match_tyme() {
        let cases = [
            (
                2020,
                4,
                30,
                SolarDate {
                    year: 2020,
                    month: 4,
                    day: 23,
                },
            ),
            (
                2020,
                -4,
                29,
                SolarDate {
                    year: 2020,
                    month: 5,
                    day: 23,
                },
            ),
            (
                2023,
                6,
                29,
                SolarDate {
                    year: 2023,
                    month: 7,
                    day: 18,
                },
            ),
            (
                2023,
                7,
                30,
                SolarDate {
                    year: 2023,
                    month: 8,
                    day: 16,
                },
            ),
            (
                2023,
                8,
                30,
                SolarDate {
                    year: 2023,
                    month: 9,
                    day: 15,
                },
            ),
            (
                2023,
                9,
                29,
                SolarDate {
                    year: 2023,
                    month: 10,
                    day: 15,
                },
            ),
            (
                2033,
                -11,
                29,
                SolarDate {
                    year: 2033,
                    month: 12,
                    day: 22,
                },
            ),
            (
                2033,
                12,
                30,
                SolarDate {
                    year: 2034,
                    month: 1,
                    day: 20,
                },
            ),
            (
                9_999,
                12,
                29,
                SolarDate {
                    year: 9_999,
                    month: 12,
                    day: 30,
                },
            ),
        ];

        for (year, month, days, first_day) in cases {
            let lunar_month = LunarMonth::from_ym(year, month).unwrap();
            assert_eq!(lunar_month.day_count().unwrap(), days, "{year}-{month}");
            assert_eq!(
                lunar_month.first_solar_date().unwrap(),
                first_day,
                "{year}-{month}"
            );
        }
    }

    #[test]
    fn next_month_matches_tyme_leap_navigation() {
        assert_eq!(
            LunarMonth::from_ym(2008, 11)
                .unwrap()
                .next(1)
                .unwrap()
                .month_with_leap(),
            12
        );
        assert_eq!(
            LunarMonth::from_ym(2008, 11)
                .unwrap()
                .next(2)
                .unwrap()
                .month_with_leap(),
            1
        );
        assert_eq!(
            LunarMonth::from_ym(2020, 4)
                .unwrap()
                .next(1)
                .unwrap()
                .month_with_leap(),
            -4
        );
        assert_eq!(
            LunarMonth::from_ym(2020, -4)
                .unwrap()
                .next(1)
                .unwrap()
                .month_with_leap(),
            5
        );
    }
}
