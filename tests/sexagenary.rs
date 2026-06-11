use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch, StemBranchError};

// --- try_new: parity validation --------------------------------------------

#[test]
fn try_new_accepts_matching_parity() {
    let sb = StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).unwrap();
    assert_eq!(sb.stem(), HeavenlyStem::Jia);
    assert_eq!(sb.branch(), EarthlyBranch::Zi);
}

#[test]
fn try_new_rejects_mismatched_parity() {
    let err = StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chou).unwrap_err();
    assert_eq!(
        err,
        StemBranchError::InvalidStemBranchPair {
            stem: HeavenlyStem::Jia,
            branch: EarthlyBranch::Chou,
        }
    );
}

#[test]
fn try_new_admits_exactly_sixty_pairs() {
    let mut valid = 0;
    for stem in lunar_lite::HEAVENLY_STEMS {
        for branch in lunar_lite::EARTHLY_BRANCHES {
            if StemBranch::try_new(stem, branch).is_ok() {
                valid += 1;
            }
        }
    }
    // Only 60 of the 10 x 12 = 120 combinations share stem/branch parity.
    assert_eq!(valid, 60);
}

// --- from_cycle_index: canonical anchors and wrapping ----------------------

#[test]
fn cycle_index_anchors() {
    let jiazi = StemBranch::from_cycle_index(0);
    assert_eq!(jiazi.stem(), HeavenlyStem::Jia);
    assert_eq!(jiazi.branch(), EarthlyBranch::Zi);

    let yichou = StemBranch::from_cycle_index(1);
    assert_eq!(yichou.stem(), HeavenlyStem::Yi);
    assert_eq!(yichou.branch(), EarthlyBranch::Chou);

    let guihai = StemBranch::from_cycle_index(59);
    assert_eq!(guihai.stem(), HeavenlyStem::Gui);
    assert_eq!(guihai.branch(), EarthlyBranch::Hai);
}

#[test]
fn from_cycle_index_wraps_modulo_sixty() {
    assert_eq!(
        StemBranch::from_cycle_index(60),
        StemBranch::from_cycle_index(0)
    );
    assert_eq!(
        StemBranch::from_cycle_index(61),
        StemBranch::from_cycle_index(1)
    );
    assert_eq!(
        StemBranch::from_cycle_index(120 + 7),
        StemBranch::from_cycle_index(7)
    );
}

#[test]
fn from_cycle_index_only_yields_valid_pairs() {
    for index in 0..60 {
        let sb = StemBranch::from_cycle_index(index);
        // Every cycle position must itself be a parity-valid pair.
        assert!(StemBranch::try_new(sb.stem(), sb.branch()).is_ok());
    }
}

// --- cycle_index: round-trip with from_cycle_index -------------------------

#[test]
fn cycle_index_round_trips_for_all_sixty_positions() {
    for index in 0..60 {
        let sb = StemBranch::from_cycle_index(index);
        assert_eq!(sb.cycle_index(), index, "round-trip failed at {index}");
    }
}

#[test]
fn distinct_positions_are_distinct_pairs() {
    let mut seen = std::collections::HashSet::new();
    for index in 0..60 {
        let sb = StemBranch::from_cycle_index(index);
        assert!(seen.insert(sb), "duplicate stem-branch at index {index}");
    }
    assert_eq!(seen.len(), 60);
}

// --- from_lunar_year: 1984 = JiaZi anchor ----------------------------------

#[test]
fn lunar_year_anchor_1984_is_jiazi() {
    let sb = StemBranch::from_lunar_year(1984);
    assert_eq!(sb.stem(), HeavenlyStem::Jia);
    assert_eq!(sb.branch(), EarthlyBranch::Zi);
    assert_eq!(sb.cycle_index(), 0);
}

#[test]
fn lunar_year_known_regression_anchors() {
    // 2023 = 癸卯 (Gui Mao), 2024 = 甲辰 (Jia Chen) — well-known recent years.
    let y2023 = StemBranch::from_lunar_year(2023);
    assert_eq!(y2023.stem(), HeavenlyStem::Gui);
    assert_eq!(y2023.branch(), EarthlyBranch::Mao);

    let y2024 = StemBranch::from_lunar_year(2024);
    assert_eq!(y2024.stem(), HeavenlyStem::Jia);
    assert_eq!(y2024.branch(), EarthlyBranch::Chen);
}

#[test]
fn lunar_year_repeats_every_sixty_years() {
    for year in 1900..1960 {
        assert_eq!(
            StemBranch::from_lunar_year(year),
            StemBranch::from_lunar_year(year + 60),
        );
    }
}

#[test]
fn lunar_year_handles_years_before_anchor() {
    // 1983 is one step before the 1984 JiaZi anchor: index 59 = 癸亥.
    let sb = StemBranch::from_lunar_year(1983);
    assert_eq!(sb.cycle_index(), 59);
    assert_eq!(sb.stem(), HeavenlyStem::Gui);
    assert_eq!(sb.branch(), EarthlyBranch::Hai);
}

#[cfg(feature = "serde")]
#[test]
fn stem_branch_round_trips_through_json() {
    let sb = StemBranch::from_lunar_year(2024);
    let json = serde_json::to_string(&sb).unwrap();
    let back: StemBranch = serde_json::from_str(&json).unwrap();
    assert_eq!(sb, back);
}
