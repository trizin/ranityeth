pub fn is_possible_pattern(x: &str) -> bool {
    x.as_bytes()
        .iter()
        .all(|&c| (b'a'..=b'f').contains(&c) || (b'0'..=b'9').contains(&c))
}

pub fn calculate_difficulty(s: &str, case_sensitive: bool) -> u64 {
    if case_sensitive {
        22_u64.pow(s.len() as u32)
    } else {
        16_u64.pow(s.len() as u32)
    }
}

pub fn calculate_estimated_time(speed: u64, difficulty: u64) -> u64 {
    if speed == 0 {
        0
    } else {
        difficulty / speed
    }
}

pub fn time_left(estimated_time: u64, elapsed_time: u64) -> u64 {
    if elapsed_time > estimated_time {
        return 0;
    }
    estimated_time - elapsed_time
}
