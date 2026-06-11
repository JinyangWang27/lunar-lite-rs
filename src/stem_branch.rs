/// Canonical cyclic ordering of the ten Heavenly Stems.
pub const HEAVENLY_STEMS: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia,
    HeavenlyStem::Yi,
    HeavenlyStem::Bing,
    HeavenlyStem::Ding,
    HeavenlyStem::Wu,
    HeavenlyStem::Ji,
    HeavenlyStem::Geng,
    HeavenlyStem::Xin,
    HeavenlyStem::Ren,
    HeavenlyStem::Gui,
];

/// Canonical cyclic ordering of the twelve Earthly Branches.
pub const EARTHLY_BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch::Zi,
    EarthlyBranch::Chou,
    EarthlyBranch::Yin,
    EarthlyBranch::Mao,
    EarthlyBranch::Chen,
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Shen,
    EarthlyBranch::You,
    EarthlyBranch::Xu,
    EarthlyBranch::Hai,
];

/// One of the ten Heavenly Stems.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HeavenlyStem {
    /// Jia stem (甲).
    Jia,
    /// Yi stem (乙).
    Yi,
    /// Bing stem (丙).
    Bing,
    /// Ding stem (丁).
    Ding,
    /// Wu stem (戊).
    Wu,
    /// Ji stem (己).
    Ji,
    /// Geng stem (庚).
    Geng,
    /// Xin stem (辛).
    Xin,
    /// Ren stem (壬).
    Ren,
    /// Gui stem (癸).
    Gui,
}

impl HeavenlyStem {
    /// Returns this stem's zero-based position in [`HEAVENLY_STEMS`].
    pub const fn index(self) -> usize {
        match self {
            Self::Jia => 0,
            Self::Yi => 1,
            Self::Bing => 2,
            Self::Ding => 3,
            Self::Wu => 4,
            Self::Ji => 5,
            Self::Geng => 6,
            Self::Xin => 7,
            Self::Ren => 8,
            Self::Gui => 9,
        }
    }

    /// Returns the stem at `index`, wrapping with modulo arithmetic.
    pub fn from_index(index: usize) -> Self {
        HEAVENLY_STEMS[index % HEAVENLY_STEMS.len()]
    }

    /// Returns the stem offset by `delta`, wrapping in both directions.
    pub fn offset(self, delta: isize) -> Self {
        let len = HEAVENLY_STEMS.len() as isize;
        let index = (self.index() as isize + delta).rem_euclid(len) as usize;
        Self::from_index(index)
    }
}

/// One of the twelve Earthly Branches.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EarthlyBranch {
    /// Zi branch (子).
    Zi,
    /// Chou branch (丑).
    Chou,
    /// Yin branch (寅).
    Yin,
    /// Mao branch (卯).
    Mao,
    /// Chen branch (辰).
    Chen,
    /// Si branch (巳).
    Si,
    /// Wu branch (午).
    Wu,
    /// Wei branch (未).
    Wei,
    /// Shen branch (申).
    Shen,
    /// You branch (酉).
    You,
    /// Xu branch (戌).
    Xu,
    /// Hai branch (亥).
    Hai,
}

impl EarthlyBranch {
    /// Returns this branch's zero-based position in [`EARTHLY_BRANCHES`].
    pub const fn index(self) -> usize {
        match self {
            Self::Zi => 0,
            Self::Chou => 1,
            Self::Yin => 2,
            Self::Mao => 3,
            Self::Chen => 4,
            Self::Si => 5,
            Self::Wu => 6,
            Self::Wei => 7,
            Self::Shen => 8,
            Self::You => 9,
            Self::Xu => 10,
            Self::Hai => 11,
        }
    }

    /// Returns the branch at `index`, wrapping with modulo arithmetic.
    pub fn from_index(index: usize) -> Self {
        EARTHLY_BRANCHES[index % EARTHLY_BRANCHES.len()]
    }

    /// Returns the branch offset by `delta`, wrapping in both directions.
    pub fn offset(self, delta: isize) -> Self {
        let len = EARTHLY_BRANCHES.len() as isize;
        let index = (self.index() as isize + delta).rem_euclid(len) as usize;
        Self::from_index(index)
    }
}
