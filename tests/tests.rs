extern crate ffxiv_chronowatcher;

use ffxiv_chronowatcher::eorzean_time::{convert_to_eorzean_date, EorzeanDate};

mod time_tests {
    use super::*;
    #[test]
    fn test_convert_to_eorzean_date() {
        let unix_timestamp = 1661114514;
        let eorzean_date = convert_to_eorzean_date(unix_timestamp);
        assert_eq!(eorzean_date.years, 1);
        assert_eq!(eorzean_date.suns, 12);
        assert_eq!(eorzean_date.bells, 12);
        assert_eq!(eorzean_date.minutes, 4);
        assert_eq!(eorzean_date.guardian, "Halone");
        assert_eq!(eorzean_date.phase, "Waxing Gibbous");
        assert_eq!(eorzean_date.moon, "First Astral Moon");
    }
}

