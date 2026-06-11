use lunar_lite::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};

// --- Canonical ordering tables ---------------------------------------------

#[test]
fn heavenly_stem_index_matches_table_position() {
    for (position, stem) in HEAVENLY_STEMS.iter().enumerate() {
        assert_eq!(stem.index(), position, "stem {stem:?} out of order");
    }
}

#[test]
fn earthly_branch_index_matches_table_position() {
    for (position, branch) in EARTHLY_BRANCHES.iter().enumerate() {
        assert_eq!(branch.index(), position, "branch {branch:?} out of order");
    }
}

#[test]
fn tables_have_expected_lengths() {
    assert_eq!(HEAVENLY_STEMS.len(), 10);
    assert_eq!(EARTHLY_BRANCHES.len(), 12);
}

#[test]
fn first_and_last_table_entries_are_anchored() {
    assert_eq!(HEAVENLY_STEMS[0], HeavenlyStem::Jia);
    assert_eq!(HEAVENLY_STEMS[9], HeavenlyStem::Gui);
    assert_eq!(EARTHLY_BRANCHES[0], EarthlyBranch::Zi);
    assert_eq!(EARTHLY_BRANCHES[11], EarthlyBranch::Hai);
}

// --- from_index: wrapping round-trip ---------------------------------------

#[test]
fn heavenly_stem_from_index_round_trips() {
    for stem in HEAVENLY_STEMS {
        assert_eq!(HeavenlyStem::from_index(stem.index()), stem);
    }
}

#[test]
fn earthly_branch_from_index_round_trips() {
    for branch in EARTHLY_BRANCHES {
        assert_eq!(EarthlyBranch::from_index(branch.index()), branch);
    }
}

#[test]
fn heavenly_stem_from_index_wraps_modulo_ten() {
    assert_eq!(HeavenlyStem::from_index(10), HeavenlyStem::Jia);
    assert_eq!(HeavenlyStem::from_index(11), HeavenlyStem::Yi);
    assert_eq!(HeavenlyStem::from_index(25), HeavenlyStem::Ji);
}

#[test]
fn earthly_branch_from_index_wraps_modulo_twelve() {
    assert_eq!(EarthlyBranch::from_index(12), EarthlyBranch::Zi);
    assert_eq!(EarthlyBranch::from_index(13), EarthlyBranch::Chou);
    assert_eq!(EarthlyBranch::from_index(25), EarthlyBranch::Chou);
}

// --- offset: wrapping in both directions -----------------------------------

#[test]
fn heavenly_stem_offset_advances_and_wraps() {
    assert_eq!(HeavenlyStem::Jia.offset(0), HeavenlyStem::Jia);
    assert_eq!(HeavenlyStem::Jia.offset(1), HeavenlyStem::Yi);
    assert_eq!(HeavenlyStem::Gui.offset(1), HeavenlyStem::Jia);
    assert_eq!(HeavenlyStem::Jia.offset(10), HeavenlyStem::Jia);
}

#[test]
fn heavenly_stem_offset_handles_negative_deltas() {
    assert_eq!(HeavenlyStem::Jia.offset(-1), HeavenlyStem::Gui);
    assert_eq!(HeavenlyStem::Jia.offset(-11), HeavenlyStem::Gui);
    assert_eq!(HeavenlyStem::Yi.offset(-2), HeavenlyStem::Jia.offset(-1));
}

#[test]
fn earthly_branch_offset_advances_and_wraps() {
    assert_eq!(EarthlyBranch::Zi.offset(0), EarthlyBranch::Zi);
    assert_eq!(EarthlyBranch::Zi.offset(1), EarthlyBranch::Chou);
    assert_eq!(EarthlyBranch::Hai.offset(1), EarthlyBranch::Zi);
    assert_eq!(EarthlyBranch::Zi.offset(12), EarthlyBranch::Zi);
}

#[test]
fn earthly_branch_offset_handles_negative_deltas() {
    assert_eq!(EarthlyBranch::Zi.offset(-1), EarthlyBranch::Hai);
    assert_eq!(EarthlyBranch::Zi.offset(-13), EarthlyBranch::Hai);
}

#[test]
fn offset_is_consistent_across_full_cycle() {
    for stem in HEAVENLY_STEMS {
        for delta in -25..=25 {
            assert_eq!(
                stem.offset(delta),
                HeavenlyStem::from_index((stem.index() as isize + delta).rem_euclid(10) as usize),
            );
        }
    }
    for branch in EARTHLY_BRANCHES {
        for delta in -25..=25 {
            assert_eq!(
                branch.offset(delta),
                EarthlyBranch::from_index((branch.index() as isize + delta).rem_euclid(12) as usize),
            );
        }
    }
}
