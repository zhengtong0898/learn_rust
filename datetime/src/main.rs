use chrono::{FixedOffset, LocalResult, TimeZone, Utc};

pub fn datetime_from_timestamp(timestamp: i64, shanghai_timezone: bool) -> Option<String> {
    if timestamp.to_string().len() != 13 {
        eprintln!("Error: 当前函数仅支持毫秒长度的时间戳.");
        return None;
    }

    let seconds = timestamp / 1000;
    let nanos = (timestamp % 1000) * 1_000_000;

    match Utc.timestamp_opt(seconds, nanos as u32) {
        LocalResult::Single(datetime) => {
            let mut result = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

            if shanghai_timezone {
                if let Some(shanghai_offset) = FixedOffset::east_opt(8 * 3600) {
                    result = datetime
                        .with_timezone(&shanghai_offset)
                        .format("%Y-%m-%d %H:%M:%S.%9f")
                        .to_string();
                }
            }
            Some(result)
        }
        _ => {
            println!("datetime_from_timestamp: 时间戳转时间失败.");
            None
        }
    }
}

fn main() {
    // 不加时区
    if let Some(datetime_str) = datetime_from_timestamp(1701303000123, false) {
        println!("datetime_str: {}", datetime_str);
    }

    // 加上时区
    if let Some(datetime_str) = datetime_from_timestamp(1701303000153123, true) {
        println!("datetime_str: {}", datetime_str);
    }
}
