use chrono::prelude::DateTime;
use chrono::Local;
use std::time::{Duration, UNIX_EPOCH};

pub fn convert_timestamp(timestamp: i64) -> String {
    convert_timestamp_formatted(timestamp, "%d.%m.%Y")
}

pub fn convert_timestamp_formatted(timestamp: i64, format: &str) -> String {
    let date = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let datetime = DateTime::<Local>::from(date);
    datetime.format(format).to_string()
}

pub fn current_local_time_formatted(format: &str) -> String {
    let dt = Local::now();
    let timestamp: i64 = dt.timestamp();
    convert_timestamp_formatted(timestamp, format)
}

#[cfg(test)]
mod tests {

    #[test]
    fn convert_timestamp_formatted_test() {
        use crate::utils::convert_timestamp_formatted;
        let left = "12.12.2021";
        let right = convert_timestamp_formatted(1639267200, "%d.%m.%Y");
        assert_eq!(left, right);
    }

    #[test]
    fn convert_timestamp_test() {
        use crate::utils::convert_timestamp;
        let left = "12.12.2021";
        let right = convert_timestamp(1639267200);
        assert_eq!(left, right);
    }

    #[test]
    fn current_local_time_formatted_test() {
        use crate::utils::current_local_time_formatted;
        use chrono::prelude::DateTime;
        use chrono::Local;
        use std::time::{Duration, UNIX_EPOCH};

        let format = "%d.%m.%Y";
        let dt = Local::now();
        let timestamp: i64 = dt.timestamp();
        let date = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
        let datetime = DateTime::<Local>::from(date);
        let left = datetime.format(format).to_string();
        let right = current_local_time_formatted(format);
        assert_eq!(left, right);
    }
}
