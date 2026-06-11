use crate::LunarError;

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
