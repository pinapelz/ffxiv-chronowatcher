use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

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

/// Handles i64 and DateTime<Utc> types
pub trait ToUnixTimestamp {
    /// Converts the implementing type to a Unix timestamp.
    ///
    /// # Returns
    /// 
    /// An `i64` representing the Unix timestamp
    fn to_unix_timestamp(&self) -> i64;
}

impl ToUnixTimestamp for i64 {
    /// Returns the `i64` value as the Unix timestamp
    fn to_unix_timestamp(&self) -> i64 {
        *self
    }
}

impl ToUnixTimestamp for DateTime<Utc> {
    /// Converts `DateTime<Utc>` to a Unix timestamp in milliseconds
    fn to_unix_timestamp(&self) -> i64 {
        self.timestamp_millis()
    }
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
        println!("Weather: {}, Chance: {}", weather, chance);
        if forecast_target < chance {
            return map_string_to_weather(weather);
        }
    }

    // Throw an error if no weather found
    panic!("No weather found for the forecast target");
}

