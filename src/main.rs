mod eorzean_time;

use chrono::Utc;
use eorzean_time::convert_seconds_to_eorzean_duration;


fn main() {
    let current_time = Utc::now().timestamp_millis();
    let eorzean_date = eorzean_time::convert_to_eorzean_date(current_time);
    println!("{:?}", eorzean_date);
}