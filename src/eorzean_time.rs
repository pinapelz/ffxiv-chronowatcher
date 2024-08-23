use chrono::{DateTime,Utc};

const EORZEA_CONSTANT: f64 = 3600.0 / 175.0;

// General conversion rates
pub const EORZEA_SECONDS_PER_MINUTE: f64 = 60.0;
pub const EORZEA_SECONDS_PER_HOUR: f64 = 60.0 * EORZEA_SECONDS_PER_MINUTE;
pub const EORZEA_SECONDS_PER_SUN: f64 = 24.0 * EORZEA_SECONDS_PER_HOUR;
pub const EORZEA_SECONDS_PER_WEEK: f64 = 8.0 * EORZEA_SECONDS_PER_SUN;
pub const EORZEA_SECONDS_PER_MOON: f64 = 4.0 * EORZEA_SECONDS_PER_WEEK;
pub const EORZEA_SECONDS_PER_YEAR: f64 = 12.0 * EORZEA_SECONDS_PER_MOON;

pub const EORZEA_THE_TWELVE: [&str; 12] = [
    "Halone",
    "Menphina",
    "Thaliak",
    "Nymeia",
    "Llymlaen",
    "Oschon",
    "Byregot",
    "Rhalgr",
    "Azeyma",
    "Nald'thal",
    "Nophica",
    "Althyk",
];

pub const EORZEA_MOON_PHASES: [&str; 8] = [
    "New Moon",
    "Waxing Crescent",
    "Waxing Half Moon",
    "Waxing Gibbous",
    "Full Moon",
    "Waning Gibbous",
    "Waning Half Moon",
    "Waning Crescent",
];

pub const EORZEA_MOON_CYCLE_PREFIX: [&str; 6] = [
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth"
];


#[derive(Debug)]
pub struct EorzeanTime {
    pub years: u64,
    pub moons: u64,
    pub weeks: u64,
    pub suns: u64,
    pub bells: u64,
    pub minutes: u64,
    pub seconds: u64,
}

#[derive(Debug)]
pub struct EorzeanDate {
    pub guardian: String,
    pub phase: String,
    pub moon: String,
    pub years: u64,
    pub suns: u64,
    pub bells: u64,
    pub minutes: u64,
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

/// Calculates the Eorzean time given a Unix timestamp
/// 
/// # Arguments
/// - `input_time` - A type that implements the `ToUnixTimestamp` trait (i64 or chrono::DateTime<Utc>)
/// 
/// # Returns
/// - An EorzeanDate struct representing the Eorzean time equivalent of the input time
pub fn convert_to_eorzean_date<T: ToUnixTimestamp>(input_time: T) -> EorzeanDate {
    let local_epoch = input_time.to_unix_timestamp();
    
    let epoch = local_epoch as f64 * EORZEA_CONSTANT;
    let minutes = (epoch / (1000.0 * EORZEA_SECONDS_PER_MINUTE)) % EORZEA_SECONDS_PER_MINUTE;
    let bells = (epoch / (1000.0 * EORZEA_SECONDS_PER_HOUR)) % 24.0;
    let total_suns = (epoch / (1000.0 * EORZEA_SECONDS_PER_SUN)) as u64;
    let year = total_suns / (32 * 12);
    let moon_idx = (total_suns / 32) % 12 + 1;
    let mut moon_str = EORZEA_MOON_CYCLE_PREFIX[(moon_idx / 2) as usize].to_string();
    
    if moon_idx % 2 == 0 {
        moon_str.push_str(" Umbral Moon");
    } else {
        moon_str.push_str(" Astral Moon");
    }
    
    let sun = total_suns % 32 + 1;
    let guardian = EORZEA_THE_TWELVE[(moon_idx - 1) as usize].to_string();
    let moon_phase = EORZEA_MOON_PHASES[(sun / 4) as usize].to_string();
    
    EorzeanDate {
        guardian: guardian,
        phase: moon_phase,
        moon: moon_str,
        years: year as u64,
        suns: sun as u64,
        bells: bells as u64,
        minutes: minutes as u64,
    }
}
/// Converts a Unix timestamp to Eorzean time
/// A much simpler function to quickly get the current Eorzean time
/// 
/// # Arguments
/// - `input_time` - A type that implements the `ToUnixTimestamp` trait (i64 or chrono::DateTime<Utc>)
/// 
/// # Returns
/// - A tuple containing the bells and minutes of the Eorzean time equivalent of the input time
pub fn convert_to_eorzean_time<T: ToUnixTimestamp>(input_time: T) -> (u8, u8) {
    let local_epoch = input_time.to_unix_timestamp();
    
    let epoch = local_epoch as f64 * EORZEA_CONSTANT;
    let minutes = (epoch / (1000.0 * 60.0)) % 60.0;
    let bells = (epoch / (1000.0 * 60.0 * 60.0)) % 24.0;
    (bells as u8, minutes as u8)
}

/// Converts seconds to an EorzeanTime struct
///
/// # Arguments
/// - `seconds` - A `f64` representing the number of seconds to convert
/// 
/// # Returns
/// - An `EorzeanTime` struct representing the Eorzean time equivalent of the input seconds
pub fn convert_earth_seconds_to_eorzean_duration(seconds: f64) -> EorzeanTime {
    let eorzean_seconds = seconds * 22.0/12.0;

    let years = (eorzean_seconds / EORZEA_SECONDS_PER_YEAR) as u64;
    let remaining_seconds = eorzean_seconds % EORZEA_SECONDS_PER_YEAR;

    let months = (remaining_seconds / EORZEA_SECONDS_PER_MOON) as u64;
    let remaining_seconds = remaining_seconds % EORZEA_SECONDS_PER_MOON;

    let weeks = (remaining_seconds / EORZEA_SECONDS_PER_WEEK) as u64;
    let remaining_seconds = remaining_seconds % EORZEA_SECONDS_PER_WEEK;

    let days = (remaining_seconds / EORZEA_SECONDS_PER_SUN) as u64;
    let remaining_seconds = remaining_seconds % EORZEA_SECONDS_PER_SUN;

    let bells = (remaining_seconds / EORZEA_SECONDS_PER_HOUR) as u64;
    let remaining_seconds = remaining_seconds % EORZEA_SECONDS_PER_HOUR;

    let minutes = (remaining_seconds / EORZEA_SECONDS_PER_MINUTE) as u64;
    let seconds = remaining_seconds % EORZEA_SECONDS_PER_MINUTE;

    EorzeanTime {
        years,
        moons: months,
        weeks,
        suns: days,
        bells,
        minutes,
        seconds: seconds as u64,
    }
}

/// Converts an EorzeanDuration to Earth seconds
/// 
/// # Arguments
/// - `eorzean_duration` - An `EorzeanTime` struct representing the Eorzean time equivalent of the input seconds
/// 
/// # Returns
/// - A `i64` representing the number of seconds to convert
pub fn convert_eorzean_duration_to_earth_seconds(eorzean_duration: EorzeanTime) -> i64 {
    let total_seconds = eorzean_duration.years as f64 * EORZEA_SECONDS_PER_YEAR +
        eorzean_duration.moons as f64 * EORZEA_SECONDS_PER_MOON +
        eorzean_duration.weeks as f64 * EORZEA_SECONDS_PER_WEEK +
        eorzean_duration.suns as f64 * EORZEA_SECONDS_PER_SUN +
        eorzean_duration.bells as f64 * EORZEA_SECONDS_PER_HOUR +
        eorzean_duration.minutes as f64 * EORZEA_SECONDS_PER_MINUTE +
        eorzean_duration.seconds as f64;
    (total_seconds as i64 * 12/22) as i64
}
