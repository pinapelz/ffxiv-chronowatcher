use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;
use crate::eorzean_time::convert_to_eorzean_time;
use crate::eorzean_time::eorzea_duration_to_earth_sec;
use crate::eorzean_time::EorzeanTime;
use crate::eorzean_time::ToUnixTimestamp;

/// The different weather types in the game
#[derive(Debug, PartialEq)]
pub enum Weather{
    AstroMagneticStorm,
    Blizzards,
    ClearSkies,
    Clouds,
    DustStorms,
    FairSkies,
    Fog,
    Gales,
    Gloom,
    HeatWaves,
    MoonDust,
    Rain,
    Showers,
    Snow,
    Thunder,
    Thunderstorms,
    UmbralStatic,
    UmbralWind,
    Wind
}

impl Weather{
    pub fn to_string(&self) -> String{
        match self{
            Weather::AstroMagneticStorm => "Astro-Magnetic Storms".to_string(),
            Weather::Blizzards => "Blizzards".to_string(),
            Weather::ClearSkies => "Clear Skies".to_string(),
            Weather::Clouds => "Clouds".to_string(),
            Weather::DustStorms => "Dust Storms".to_string(),
            Weather::FairSkies => "Fair Skies".to_string(),
            Weather::Fog => "Fog".to_string(),
            Weather::Gales => "Gales".to_string(),
            Weather::Gloom => "Gloom".to_string(),
            Weather::HeatWaves => "Heat Waves".to_string(),
            Weather::MoonDust => "Moon Dust".to_string(),
            Weather::Rain => "Rain".to_string(),
            Weather::Showers => "Showers".to_string(),
            Weather::Snow => "Snow".to_string(),
            Weather::Thunder => "Thunder".to_string(),
            Weather::Thunderstorms => "Thunderstorms".to_string(),
            Weather::UmbralStatic => "Umbral Static".to_string(),
            Weather::UmbralWind => "Umbral Wind".to_string(),
            Weather::Wind => "Wind".to_string()
        }
    }
}

fn map_string_to_weather(zone_name: String) -> Weather {
    match zone_name.as_str() {
        "Astro-Magnetic Storms" => Weather::AstroMagneticStorm,
        "Blizzards" => Weather::Blizzards,
        "Clear Skies" => Weather::ClearSkies,
        "Clouds" => Weather::Clouds,
        "Dust Storms" => Weather::DustStorms,
        "Fair Skies" => Weather::FairSkies,
        "Fog" => Weather::Fog,
        "Gales" => Weather::Gales,
        "Gloom" => Weather::Gloom,
        "Heat Waves" => Weather::HeatWaves,
        "Moon Dust" => Weather::MoonDust,
        "Rain" => Weather::Rain,
        "Showers" => Weather::Showers,
        "Snow" => Weather::Snow,
        "Thunder" => Weather::Thunder,
        "Thunderstorms" => Weather::Thunderstorms,
        "Umbral Static" => Weather::UmbralStatic,
        "Umbral Wind" => Weather::UmbralWind,
        "Wind" => Weather::Wind,
        _ => panic!("Invalid weather type")
    }
}

#[derive(Debug)]
pub struct EorzeaWeather{
    pub start_time: i64,
    pub end_time: i64,
    pub zone_name: String,
    pub weather: Weather
}

/// Calculates the current weather interval
/// 
/// # Arguments
/// - `current_time` - The current time to calculate the forecast for
/// 
/// # Returns
/// - A tuple containing the start and end times of the current weather interval
pub fn calculate_current_weather_interval<T: ToUnixTimestamp>(current_time: T) -> (i64, i64){
    let current_epoch = current_time.to_unix_timestamp();
    let current_eorzean_time = convert_to_eorzean_time(current_epoch);
    let curr_hour = current_eorzean_time.0 as i64;
    let curr_minute = current_eorzean_time.1 as i64;
    // Find the nearest 8 Eorzean hour interval (00:00, 08:00, 16:00)
    let nearest_start_interval_hour = (curr_hour / 8) * 8;
    let nearest_end_interval_hour = ((curr_hour / 8) + 1) * 8;

    // Find the difference in minutes to the nearest start and end intervals
    let ezt_minutes_since_start_interval = (curr_hour * 60 + curr_minute) - (nearest_start_interval_hour * 60);
    let ezt_minutes_to_end_interval = if nearest_end_interval_hour == 24 {
        (24 * 60) - (curr_hour * 60 + curr_minute)
    } else {
        (nearest_end_interval_hour * 60) - (curr_hour * 60 + curr_minute)
    };

 
    let seconds_since_start_interval = eorzea_duration_to_earth_sec(
        EorzeanTime {
            years: 0,
            moons: 0,
            weeks: 0,
            suns: 0,
            bells: 0,
            minutes: ezt_minutes_since_start_interval as u64,
            seconds: 0,
        }
    );

    let seconds_to_end_interval = eorzea_duration_to_earth_sec(
        EorzeanTime {
            years: 0,
            moons: 0,
            weeks: 0,
            suns: 0,
            bells: 0,
            minutes: ezt_minutes_to_end_interval as u64,
            seconds: 0,
        }
    );
    let current_epoch_seconds = current_epoch.to_unix_timestamp();
    let start_time = current_epoch_seconds - seconds_since_start_interval as i64;
    let end_time = current_epoch_seconds + seconds_to_end_interval as i64;
    (start_time, end_time)
    
}


/// Calculates the magic number used to determine the weather
/// 
/// # Arguments
/// - `current_time` - The current time to calculate the forecast for
/// 
/// # Returns
/// - An `i32` representing the magic number used to determine the weather
pub fn calculate_weather_forecast_target<T: ToUnixTimestamp>(current_time: T) -> i32 {
    // Calculate magic weather number the game uses. Thanks to ffxiv-datamining
    let unix_seconds = current_time.to_unix_timestamp();
    let bell = unix_seconds / 175;
    let increment = (bell + 8 - (bell % 8)) % 24;
    let total_days = unix_seconds / 4200;
    let total_days: u32 = total_days.try_into().unwrap_or(0);
    let calc_base = total_days * 100 + increment as u32;
    let step1 = (calc_base << 11) ^ calc_base;
    let step2 = (step1 >> 8) ^ step1;
    (step2 % 100) as i32
}

pub fn get_weather_by_time<T: ToUnixTimestamp>(zone_name: &str, current_time: T) -> Weather {
    // Load the weather data from the JSON file
    let weather_data = fs::read_to_string("data/weather_data.json").expect("Unable to read the weather data file");
    let weather_data: serde_json::Value = serde_json::from_str(&weather_data).expect("Unable to parse the weather data");
    let zone_data = weather_data
        .get(zone_name)
        .and_then(|zone| zone.as_array())
        .expect(&format!("Unable to find the zone '{}' in the weather data", zone_name));

    // Convert the JSON array to a Vec of (String, i32) tuples
    let zone_data = zone_data
        .iter()
        .map(|entry| {
            let weather = entry[0].as_str().expect("Invalid weather format").to_string();
            let chance = entry[1].as_i64().expect("Invalid chance format") as i32;
            (weather, chance)
        })
        .collect::<Vec<(String, i32)>>();

    // Get the current forecast target
    let epoch = current_time.to_unix_timestamp();
    let forecast_target = calculate_weather_forecast_target(epoch);

    // Find the weather type that matches the forecast target
    for (weather, chance) in zone_data {
        if forecast_target < chance {
            return map_string_to_weather(weather);
        }
    }

    // Throw an error if no weather found
    panic!("No weather found for the forecast target");
}

/// Calculates the weather forecast for a given zone at a given time
/// 
/// # Arguments
/// - `zone_name` - The name of the zone to calculate the forecast for
/// - `current_time` - The current time to calculate the forecast for
/// - `offset` - The intervals to calculate the forecast for. +1 means the next interval, -1 means the previous interval
/// 
/// # Returns
/// - An EorzeaWeather struct representing the forecasted weather
/// 
pub fn calculate_forecast<T: ToUnixTimestamp>(zone_name: &str, current_time: T, interval_offset: i32) -> EorzeaWeather {
    // Each interval is 8 Eorzean hours. 00:00, 08:00, 16:00 are the start times
    let current_epoch = current_time.to_unix_timestamp();
    if interval_offset == 0 {
        // Calculate the current weather interval
        let (start_time, end_time) = calculate_current_weather_interval(current_time);
        let current_weather = get_weather_by_time(zone_name, current_epoch);
        return EorzeaWeather {
            start_time: start_time,
            end_time: end_time,
            zone_name: zone_name.to_string(),
            weather: current_weather
        };
    }
    
    // Find the current interval and get the weather for the start of the interval
    let current_forecast_interval = calculate_current_weather_interval(current_epoch);
    // Weather changes every 23 real-world minutes, convert to 60 seconds, adjust for number of intervals seeking
    let offset_interval_start = current_forecast_interval.0 + (23* (1 + interval_offset as i64) *60);
    let weather_at_offset = get_weather_by_time(zone_name, offset_interval_start);
    EorzeaWeather {
        start_time: offset_interval_start - 1380,
        end_time: offset_interval_start,
        zone_name: zone_name.to_string(),
        weather: weather_at_offset
    }
}

/// Find the time which a next Weather effect will occur
/// 
/// # Arguments
/// - `zone_name` - The name of the zone to calculate the forecast for
/// - `current_time` - The current time to calculate the forecast for
/// - `target_weather` - The weather effect to search for
/// 
/// # Returns
/// - An EorzeaWeather struct representing the next weather effect
pub fn find_next_weather_occurance<T: ToUnixTimestamp>(zone_name: &str, current_time: T, target_weather: Weather) -> EorzeaWeather{
    let current_epoch = current_time.to_unix_timestamp();
    let mut current_interval = 1;
    let mut next_weather = calculate_forecast(zone_name, current_epoch, current_interval);
    while next_weather.weather != target_weather {
        current_interval += 1;
        next_weather = calculate_forecast(zone_name, current_epoch, current_interval);
    }
    next_weather
}