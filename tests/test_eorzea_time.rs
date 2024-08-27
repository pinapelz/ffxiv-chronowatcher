extern crate ffxiv_chronowatcher;

use ffxiv_chronowatcher::eorzean_time::{convert_to_eorzean_date, convert_to_eorzean_time, earth_sec_to_eorzea_duration, eorzea_duration_to_earth_sec, EorzeanTime, EorzeanDate};
use chrono::DateTime;

mod time_tests {
    use super::*;
    #[test]
    fn test_convert_to_eorzean_date_astral() {
        let unix_timestamp = 141414141414;
        let eorzean_date = convert_to_eorzean_date(unix_timestamp);
        assert_eq!(eorzean_date.years, 87682);
        assert_eq!(eorzean_date.suns, 18);
        assert_eq!(eorzean_date.bells, 16);
        assert_eq!(eorzean_date.minutes, 4);
        assert_eq!(eorzean_date.guardian, "Llymlaen");
        assert_eq!(eorzean_date.phase, "Full Moon");
        assert_eq!(eorzean_date.moon, "Third Astral Moon");
    }

    #[test]
    fn test_convert_to_ez_chrono_datetime(){
        let timestamp: i64 = 141414141414;
        let chrono_date = DateTime::from_timestamp(timestamp, 0).expect("Failed to create DateTime");
        let eorzean_date = convert_to_eorzean_date(chrono_date);
        assert_eq!(eorzean_date.years, 87682);
        assert_eq!(eorzean_date.suns, 18);
        assert_eq!(eorzean_date.bells, 16);
        assert_eq!(eorzean_date.minutes, 4);
        assert_eq!(eorzean_date.guardian, "Llymlaen");
        assert_eq!(eorzean_date.phase, "Full Moon");
        assert_eq!(eorzean_date.moon, "Third Astral Moon");
    }

    #[test]
    fn test_convert_eorzean_date_umbral() {
        let unix_timestamp = 11223344;
        let eorzean_date = convert_to_eorzean_date(unix_timestamp);
        assert_eq!(eorzean_date.years, 6);
        assert_eq!(eorzean_date.suns, 17);
        assert_eq!(eorzean_date.bells, 5);
        assert_eq!(eorzean_date.minutes, 23);
        assert_eq!(eorzean_date.guardian, "Althyk");
        assert_eq!(eorzean_date.phase, "Full Moon");
        assert_eq!(eorzean_date.moon, "Sixth Umbral Moon");
    }

    #[test]
    fn test_convert_to_eorzean_time() {
        let unix_timestamp = 1661114514;
        let time_tuple = convert_to_eorzean_time(unix_timestamp);
        assert_eq!(time_tuple.0, 10);
        assert_eq!(time_tuple.1, 56);
    }

    #[test]
    fn test_earth_sec_to_eorzea_sec(){
        let test_seconds = 70*60; // 70 minutes in seconds, should eq 1 sun = 86400 EZ sec
        let eorzean_duration = earth_sec_to_eorzea_duration(test_seconds);
        assert_eq!(eorzean_duration.suns, 1);
        assert_eq!(eorzean_duration.years, 0);
        assert_eq!(eorzean_duration.moons, 0);
        assert_eq!(eorzean_duration.weeks, 0);
        assert_eq!(eorzean_duration.bells, 0);
        assert_eq!(eorzean_duration.minutes, 0);
    }

    #[test]
    fn test_eorzea_dur_to_earth_sec() {
        let test_ez_time = EorzeanTime{
            years: 0,
            moons: 0,
            weeks: 1,
            suns: 1,
            bells: 0,
            minutes: 0,
            seconds: 0
        };
        // 37800 seconds in Earth time
        let seconds = eorzea_duration_to_earth_sec(test_ez_time);
        assert_eq!(seconds, 37800);
    }
}

