use lunar_lite::{LunarError, time_index};

/// Every double-hour (時辰) bucket, probed at the first and last minute of its
/// range. `00:00` and `23:00` are deliberately distinct buckets (early vs. late
/// 子時), so the table covers indices 0..=12.
#[test]
fn time_index_covers_every_bucket() {
    let cases = [
        (0, 0, 0),
        (0, 59, 0),
        (1, 0, 1),
        (2, 59, 1),
        (3, 0, 2),
        (4, 59, 2),
        (5, 0, 3),
        (6, 59, 3),
        (7, 0, 4),
        (8, 59, 4),
        (9, 0, 5),
        (10, 59, 5),
        (11, 0, 6),
        (12, 59, 6),
        (13, 0, 7),
        (14, 59, 7),
        (15, 0, 8),
        (16, 59, 8),
        (17, 0, 9),
        (18, 59, 9),
        (19, 0, 10),
        (20, 59, 10),
        (21, 0, 11),
        (22, 59, 11),
        (23, 0, 12),
        (23, 59, 12),
    ];

    for (hour, minute, expected) in cases {
        assert_eq!(
            time_index(hour, minute).unwrap(),
            expected,
            "time_index({hour:02}:{minute:02}) should be {expected}"
        );
    }
}

#[test]
fn time_index_rejects_invalid_hour() {
    assert_eq!(
        time_index(24, 0).unwrap_err(),
        LunarError::InvalidTime {
            hour: 24,
            minute: 0
        }
    );
}

#[test]
fn time_index_rejects_invalid_minute() {
    assert_eq!(
        time_index(23, 60).unwrap_err(),
        LunarError::InvalidTime {
            hour: 23,
            minute: 60
        }
    );
}
