extern crate ffxiv_chronowatcher;

use ffxiv_chronowatcher::eorzean_weather::{
    calculate_current_weather_interval, calculate_forecast, find_next_weather_occurance,
    get_global_weather_timing_offset, get_weather_by_time, set_global_weather_timing_offset,
    Weather,
};

mod weather_tests {
    use super::*;
    #[test]
    fn test_calculate_current_weather_interval() {
        let timestamp = 1724738458;
        let (start, end) = calculate_current_weather_interval(timestamp);
        assert_eq!(start, 1724738403);
        assert_eq!(end, 1724739802);
    }

    #[test]
    fn test_get_weather_by_time() {
        let timestamp = 1724738458;
        get_weather_by_time("Middle La Noscea", timestamp);
    }

    #[test]
    fn test_calculate_forecast_pos_offset() {
        let timestamp = 1724738458;
        let weather = calculate_forecast("Middle La Noscea", timestamp, 8);
        assert_eq!(weather.weather, Weather::ClearSkies);
        assert_eq!(weather.start_time, 1724749443);
        assert_eq!(weather.end_time, 1724750823);
        assert_eq!(weather.zone_name, "Middle La Noscea");
    }

    #[test]
    fn test_calculate_forecast_neg_offset() {
        let timestamp = 1724738458;
        let weather = calculate_forecast("Middle La Noscea", timestamp, -1);
        assert_eq!(weather.weather, Weather::Wind);
        assert_eq!(weather.start_time, 1724737023);
        assert_eq!(weather.end_time, 1724738403);
        assert_eq!(weather.zone_name, "Middle La Noscea");
    }

    #[test]
    fn test_calculate_forecast_zero_offset() {
        let timestamp = 1724738458;
        let weather = calculate_forecast("Middle La Noscea", timestamp, 0);
        assert_eq!(weather.weather, Weather::Wind);
        assert_eq!(weather.start_time, 1724738403);
        assert_eq!(weather.end_time, 1724739802);
        assert_eq!(weather.zone_name, "Middle La Noscea");
    }

    #[test]
    fn test_find_next_weather_occurance() {
        let timestamp = 1724738458;
        let weather = find_next_weather_occurance("Eureka Pagos", timestamp, Weather::Blizzards);
        assert_eq!(weather.weather, Weather::Blizzards);
        assert_eq!(weather.start_time, 1724742543);
        assert_eq!(weather.end_time, 1724743923);
        assert_eq!(weather.zone_name, "Eureka Pagos");
    }

    #[test]
    fn test_convert_weather_enum_to_string() {
        let weather_variants = vec![
            (Weather::AstroMagneticStorm, "Astro-Magnetic Storms"),
            (Weather::Blizzards, "Blizzards"),
            (Weather::ClearSkies, "Clear Skies"),
            (Weather::Clouds, "Clouds"),
            (Weather::DustStorms, "Dust Storms"),
            (Weather::FairSkies, "Fair Skies"),
            (Weather::Fog, "Fog"),
            (Weather::Gales, "Gales"),
            (Weather::Gloom, "Gloom"),
            (Weather::HeatWaves, "Heat Waves"),
            (Weather::MoonDust, "Moon Dust"),
            (Weather::Rain, "Rain"),
            (Weather::Showers, "Showers"),
            (Weather::Snow, "Snow"),
            (Weather::Thunder, "Thunder"),
            (Weather::Thunderstorms, "Thunderstorms"),
            (Weather::UmbralStatic, "Umbral Static"),
            (Weather::UmbralWind, "Umbral Wind"),
            (Weather::Wind, "Wind"),
        ];

        for (weather, expected_str) in weather_variants {
            assert_eq!(weather.to_string(), expected_str);
        }
    }

    #[test]
    fn test_calculate_current_weather_interval_day_overflow() {
        let (_, __) = calculate_current_weather_interval(1672531199);
        assert_eq!(1, 1);
    }

    #[test]
    #[should_panic]
    fn get_weather_by_time_panic_zone_not_found() {
        let timestamp = 1724738458;
        get_weather_by_time("Somewhere Not Here", timestamp);
    }

    #[test]
    fn test_set_global_offset() {
        set_global_weather_timing_offset(1724738458);
        assert_eq!(get_global_weather_timing_offset(), 1724738458);
        set_global_weather_timing_offset(0);
    }

    #[test]
    fn test_find_weather_by_time_with_offset() {
        let timestamp = 1732396696;
        set_global_weather_timing_offset(40);
        let weather = find_next_weather_occurance("Eureka Pagos", timestamp, Weather::Fog);
        assert_eq!(weather.start_time, 1732401962);
        set_global_weather_timing_offset(0);
    }
}
