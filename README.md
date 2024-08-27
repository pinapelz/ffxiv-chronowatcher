# Chronowatcher
Chronowatcher is a Rust library for getting the time and weather of locations in Final Fantasy XIV

- Retrieving Eorzean Date
```rust
use eorzean_weather::calculate_forecast;
current_time = Utc::now().timestamp();
let eorzean_date = eorzean_time::convert_to_eorzean_date(current_time);
// EorzeanDate { guardian: "Thaliak", phase: "Waxing Half Moon", moon: "Second Astral Moon", years: 1069, suns: 9, bells: 5, minutes: 54 }

// or convert to a simple form as a tuple
let eorzean_time = eorzean_time::convert_to_eorzean_time(current_time);
// (hours/bells, mins)
// (5,54)
```

- Converting durations between Eorzean and Earth time
```rust
use eorzean_time::EorzeanTime;
use eorzean_time::eorzea_duration_to_earth_sec;

let earth_seconds = eorzea_duration_to_earth_sec(
    EorzeanTime {
        years: 0,
        moons: 0,
        weeks: 0,
        suns: 1,
        bells: 2,
        minutes: 3,
        seconds: 0,
    }
);
// 4558.75
```

- Calculating Weather Forecast
```rust
use eorzean_weather::find_next_weather_occurance;
use eorzean_weather::get_weather_by_time;
use eorzean_weather::calculate_current_weather_interval;

let current_weather = get_weather_by_time("Middle La Noscea", current_time);
// Current weather: Clouds

// 3rd parameter is the number of intervals (+/-) to calculate
// 1 = what is the weather after the current one
// 2 = what is the weather in 2 changes (after the current one)
// -1 = what was the weather before the current one
let future_weather = calculate_forecast("Middle La Noscea", current_time, 1);
// Future weather: EorzeaWeather { start_time: 1724388382, end_time: 1724389762, zone_name: "Middle La Noscea", weather: FairSkies }

let when_will_it_be_rainy = find_next_weather_occurance("Middle La Noscea", current_time, eorzean_weather::Weather::Rain);
// When will it be rainy: EorzeaWeather { start_time: 1724395282, end_time: 1724396662, zone_name: "Middle La Noscea", weather: Rain }
```