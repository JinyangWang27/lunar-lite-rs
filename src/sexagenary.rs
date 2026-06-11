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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StemBranch {
    stem: HeavenlyStem,
    branch: EarthlyBranch,
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
