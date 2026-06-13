use crate::LunarError;
use crate::stem_branch::EarthlyBranch;

/// Returns the double-hour (時辰) index for a wall-clock time.
///
/// Chinese timekeeping splits the day into twelve two-hour periods. The Zi (子)
/// hour straddles midnight, so it is split: `23:00` returns `12` (late Zi) while
/// `00:00`–`00:59` returns `0` (early Zi). All other periods map to `1..=11`.
///
/// # Errors
///
/// Returns [`LunarError::InvalidTime`] if `hour > 23` or `minute > 59`.
pub fn time_index(hour: u8, minute: u8) -> Result<u8, LunarError> {
    if hour > 23 || minute > 59 {
        return Err(LunarError::InvalidTime { hour, minute });
    }

    let index = match hour {
        0 => 0,
        1 | 2 => 1,
        3 | 4 => 2,
        5 | 6 => 3,
        7 | 8 => 4,
        9 | 10 => 5,
        11 | 12 => 6,
        13 | 14 => 7,
        15 | 16 => 8,
        17 | 18 => 9,
        19 | 20 => 10,
        21 | 22 => 11,
        23 => 12,
        _ => unreachable!(),
    };

    Ok(index)
}

/// Returns the Earthly Branch (时支) for a 时辰 index.
///
/// Both `0` (early 子) and `12` (late 子) map to [`EarthlyBranch::Zi`]; all other
/// indices map to their corresponding branch via `time_index % 12`.
///
/// # Errors
///
/// Returns [`LunarError::InvalidTimeIndex`] if `time_index > 12`.
pub fn time_index_to_branch(time_index: u8) -> Result<EarthlyBranch, LunarError> {
    if time_index > 12 {
        return Err(LunarError::InvalidTimeIndex { time_index });
    }

    Ok(EarthlyBranch::from_index((time_index % 12) as usize))
}

/// Returns `true` if `time_index` is the early 子时 (`0`, 00:00–00:59).
///
/// # Errors
///
/// Returns [`LunarError::InvalidTimeIndex`] if `time_index > 12`.
pub fn is_early_zi(time_index: u8) -> Result<bool, LunarError> {
    if time_index > 12 {
        return Err(LunarError::InvalidTimeIndex { time_index });
    }

    Ok(time_index == 0)
}

/// Returns `true` if `time_index` is the late 子时 (`12`, 23:00–23:59).
///
/// # Errors
///
/// Returns [`LunarError::InvalidTimeIndex`] if `time_index > 12`.
pub fn is_late_zi(time_index: u8) -> Result<bool, LunarError> {
    if time_index > 12 {
        return Err(LunarError::InvalidTimeIndex { time_index });
    }

    Ok(time_index == 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_maps_both_zi_halves_to_zi() {
        assert_eq!(time_index_to_branch(0).unwrap(), EarthlyBranch::Zi);
        assert_eq!(time_index_to_branch(12).unwrap(), EarthlyBranch::Zi);
    }

    #[test]
    fn branch_maps_intermediate_indices() {
        assert_eq!(time_index_to_branch(1).unwrap(), EarthlyBranch::Chou);
        assert_eq!(time_index_to_branch(6).unwrap(), EarthlyBranch::Wu);
        assert_eq!(time_index_to_branch(11).unwrap(), EarthlyBranch::Hai);
    }

    #[test]
    fn early_and_late_zi_predicates() {
        assert!(is_early_zi(0).unwrap());
        assert!(!is_early_zi(12).unwrap());
        assert!(is_late_zi(12).unwrap());
        assert!(!is_late_zi(0).unwrap());
    }

    #[test]
    fn invalid_time_index_is_rejected() {
        assert_eq!(
            time_index_to_branch(13),
            Err(LunarError::InvalidTimeIndex { time_index: 13 })
        );
        assert_eq!(
            is_early_zi(13),
            Err(LunarError::InvalidTimeIndex { time_index: 13 })
        );
        assert_eq!(
            is_late_zi(13),
            Err(LunarError::InvalidTimeIndex { time_index: 13 })
        );
    }
}
