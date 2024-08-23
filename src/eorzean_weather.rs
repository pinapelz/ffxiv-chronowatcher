use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;
use crate::eorzean_time::convert_to_eorzean_time;
use crate::eorzean_time::convert_eorzean_duration_to_earth_seconds;
use crate::eorzean_time::EorzeanTime;
use crate::eorzean_time::ToUnixTimestamp;

#[derive(Debug)]
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

pub struct EorzeaWeather{
    pub start_time: i64,
    pub end_time: i64,
    pub zone_name: String,
    pub weather: Weather
}

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

 
    let seconds_since_start_interval = convert_eorzean_duration_to_earth_seconds(
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

    let seconds_to_end_interval = convert_eorzean_duration_to_earth_seconds(
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
    let current_epoch_seconds = current_epoch.to_unix_timestamp() / 1000;
    let start_time = current_epoch_seconds - seconds_since_start_interval as i64;
    let end_time = current_epoch_seconds + seconds_to_end_interval as i64;
    (start_time, end_time)
    
}


pub fn calculate_forecast_target<T: ToUnixTimestamp>(current_time: T) -> i32 {
    // Convert the input to a Unix timestamp in seconds
    let unix_seconds = current_time.to_unix_timestamp() / 1000;

    // Get Eorzea hour for weather start
    let bell = unix_seconds / 175;

    // Do the magic 'cause for calculations 16:00 is 0, 00:00 is 8 and 08:00 is 16
    let increment = (bell + 8 - (bell % 8)) % 24;

    // Take Eorzea days since unix epoch
    let total_days = unix_seconds / 4200;
    let total_days: u32 = total_days.try_into().unwrap_or(0); // Convert to uint

    // 0x64 = 100
    let calc_base = total_days * 100 + increment as u32;

    // 0xB = 11
    let step1 = (calc_base << 11) ^ calc_base;
    let step2 = (step1 >> 8) ^ step1;

    // 0x64 = 100
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
    let forecast_target = calculate_forecast_target(epoch);

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
pub fn calculate_forecast<T: ToUnixTimestamp>(zone_name: &str, current_time: T, offset: i32) -> EorzeaWeather {
    // Each interval is 8 Eorzean hours. 00:00, 08:00, 16:00 are the start times
    
    //Return a stubbed value for now
    EorzeaWeather {
        start_time: 0,
        end_time: 0,
        zone_name: zone_name.to_string(),
        weather: Weather::ClearSkies
    }
}