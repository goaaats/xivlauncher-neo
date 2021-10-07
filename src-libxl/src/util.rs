pub mod hex_slice;

pub mod time_utc {
    pub fn now_millis() -> i64 {
        chrono::Utc::now().timestamp_millis()
    }

    pub fn now_launcher_formatted() -> String {
        chrono::Utc::now().format("%Y-%m-%d-%H").to_string()
    }

    pub fn now_launcher_formatted_long() -> String {
        chrono::Utc::now().format("%Y-%m-%d-%H-%M").to_string()
    }
}