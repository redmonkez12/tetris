pub fn get_level(rows: u32) -> u32 {
    rows / 10
}

pub fn get_speed_by_level(level: u32) -> u64 {
    match level {
        0 => 800,
        1 => 720,
        2 => 630,
        3 => 550,
        4 => 470,
        5 => 380,
        6 => 300,
        7 => 220,
        8 => 130,
        9 => 100,
        10 | 11 | 12 => 80,
        13 | 14 | 15 => 70,
        16 | 17 | 18 => 50,
        19..=28 => 30,
        _ => 17,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_level_zero() {
        assert_eq!(get_level(0), 0, "Edge case: 0 rows should be level 0");
    }

    #[test]
    fn test_get_level_within_same_level() {
        // Values within the same level should return the same level
        assert_eq!(get_level(1), 0);
        assert_eq!(get_level(9), 0);
    }

    #[test]
    fn test_get_level_boundaries() {
        assert_eq!(get_level(9), 0, "Last value in level 0");
        assert_eq!(get_level(10), 1, "First value in level 1");
        assert_eq!(get_level(19), 1, "Last value in level 1");
        assert_eq!(get_level(20), 2, "First value in level 2");
    }

    #[test]
    fn test_get_level_various() {
        // Various level calculations
        assert_eq!(get_level(45), 4);
        assert_eq!(get_level(100), 10);
        assert_eq!(get_level(232), 23);
    }

    #[test]
    fn test_get_speed_lower_levels() {
        // Test specific levels at lower values
        assert_eq!(get_speed_by_level(0), 800);
        assert_eq!(get_speed_by_level(1), 720);
        assert_eq!(get_speed_by_level(5), 380);
        assert_eq!(get_speed_by_level(9), 100);
    }

    #[test]
    fn test_get_speed_grouped_levels() {
        // Test levels that share the same speed
        // Group 10-12
        assert_eq!(get_speed_by_level(10), 80);
        assert_eq!(get_speed_by_level(11), 80);
        assert_eq!(get_speed_by_level(12), 80);

        // Group 13-15
        assert_eq!(get_speed_by_level(13), 70);
        assert_eq!(get_speed_by_level(15), 70);

        // Group 16-18
        assert_eq!(get_speed_by_level(16), 50);
        assert_eq!(get_speed_by_level(18), 50);
    }

    #[test]
    fn test_get_speed_range_levels() {
        // Test the range pattern 19..=28
        assert_eq!(get_speed_by_level(19), 30);
        assert_eq!(get_speed_by_level(23), 30);
        assert_eq!(get_speed_by_level(28), 30);
    }

    #[test]
    fn test_get_speed_max_level() {
        // Test levels beyond defined ranges
        assert_eq!(get_speed_by_level(29), 17);
        assert_eq!(get_speed_by_level(100), 17);
    }
}