//! Valid sexagenary stem-branch pairs.
//!
//! A `StemBranch` represents one valid position in the sexagenary cycle
//! (六十甲子). Only sixty of the 120 possible Heavenly Stem / Earthly Branch
//! combinations are valid: the stem and branch indices must share parity.

use crate::{
    error::StemBranchError,
    stem_branch::{EarthlyBranch, HeavenlyStem},
};

/// A valid Heavenly Stem / Earthly Branch pair in the sexagenary cycle.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StemBranch {
    stem: HeavenlyStem,
    branch: EarthlyBranch,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StemBranch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawStemBranch {
            stem: HeavenlyStem,
            branch: EarthlyBranch,
        }

        let raw = RawStemBranch::deserialize(deserializer)?;
        StemBranch::try_new(raw.stem, raw.branch).map_err(serde::de::Error::custom)
    }
}
impl StemBranch {
    /// Creates a valid stem-branch pair.
    ///
    /// Returns an error if the stem and branch do not belong to the same
    /// sexagenary-cycle position.
    pub fn try_new(stem: HeavenlyStem, branch: EarthlyBranch) -> Result<Self, StemBranchError> {
        if stem.index() % 2 == branch.index() % 2 {
            Ok(Self { stem, branch })
        } else {
            Err(StemBranchError::InvalidStemBranchPair { stem, branch })
        }
    }

    /// Creates a stem-branch pair from a zero-based sexagenary-cycle index.
    ///
    /// Index `0` is JiaZi, `1` is YiChou, ..., `59` is GuiHai.
    /// The input wraps modulo 60.
    pub fn from_cycle_index(index: usize) -> Self {
        let index = index % 60;
        Self {
            stem: HeavenlyStem::from_index(index),
            branch: EarthlyBranch::from_index(index),
        }
    }

    /// Creates the stem-branch pair for a lunar year.
    ///
    /// Uses the conventional anchor `1984 = JiaZi`.
    pub fn from_lunar_year(year: i32) -> Self {
        let index = (year - 1984).rem_euclid(60) as usize;
        Self::from_cycle_index(index)
    }

    /// Returns the Heavenly Stem.
    pub const fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// Returns the Earthly Branch.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the zero-based sexagenary-cycle index.
    pub fn cycle_index(&self) -> usize {
        (0..60)
            .find(|&index| index % 10 == self.stem.index() && index % 12 == self.branch.index())
            .expect("StemBranch invariant guarantees a valid cycle index")
    }
}

/// Returns the stem-branch of a Chinese lunar year.
///
/// This is useful when a domain rule needs the lunar birth-year stem/branch
/// rather than the four-pillar year pillar, which may use a LiChun boundary.
pub fn lunar_year_stem_branch(lunar_year: i32) -> StemBranch {
    StemBranch::from_lunar_year(lunar_year)
}

/// Returns the Heavenly Stem of a Chinese lunar year (生年干).
///
/// This is useful when a domain rule needs the lunar birth-year stem rather
/// than the four-pillar year pillar, which may use a LiChun boundary.
pub fn lunar_year_stem(lunar_year: i32) -> HeavenlyStem {
    StemBranch::from_lunar_year(lunar_year).stem()
}

/// Returns the Earthly Branch of a Chinese lunar year (生年支).
///
/// This is useful when a domain rule needs the lunar birth-year branch rather
/// than the four-pillar year pillar, which may use a LiChun boundary.
pub fn lunar_year_branch(lunar_year: i32) -> EarthlyBranch {
    StemBranch::from_lunar_year(lunar_year).branch()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lunar_year_helpers_delegate_to_from_lunar_year() {
        // 2024 is 甲辰 (JiaChen).
        let expected = StemBranch::from_lunar_year(2024);
        assert_eq!(lunar_year_stem_branch(2024), expected);
        assert_eq!(lunar_year_stem(2024), HeavenlyStem::Jia);
        assert_eq!(lunar_year_branch(2024), EarthlyBranch::Chen);
    }

    #[test]
    fn lunar_year_helpers_agree_with_pillar_accessors() {
        for year in [1850, 1984, 2000, 2023, 2150] {
            let pillar = StemBranch::from_lunar_year(year);
            assert_eq!(lunar_year_stem_branch(year), pillar);
            assert_eq!(lunar_year_stem(year), pillar.stem());
            assert_eq!(lunar_year_branch(year), pillar.branch());
        }
    }
}
