use chrono::{Local, Utc};

pub fn get_current_time() {
    println!("Current UTC time is: {}", Utc::now());
    println!("Current Local time is: {}", Local::now());
}
