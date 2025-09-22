pub mod duration;
pub mod files;


pub use duration::parse_duration;

use chrono::{DateTime, FixedOffset};


pub fn replace_date_placeholder(str: String, date: DateTime<FixedOffset>) -> String {
    str.replace("${Y}", date.format("%Y").to_string().as_str())
        .replace("${y}", date.format("%y").to_string().as_str())
        .replace("${m}", date.format("%m").to_string().as_str())
        .replace(
            "${M}",
            date.format("%m").to_string().trim_start_matches("0"),
        )
        .replace("${b}", date.format("%b").to_string().as_str())
        .replace("${B}", date.format("%B").to_string().as_str())
        .replace("${d}", date.format("%d").to_string().as_str())
        .replace(
            "${D}",
            date.format("%d").to_string().trim_start_matches("0"),
        )
        .replace("${j}", date.format("%j").to_string().as_str())
        .replace(
            "${J}",
            date.format("%j").to_string().trim_start_matches("0"),
        )
        .replace("${F}", date.format("%F").to_string().as_str())
        .replace("${Ymd}", date.format("%Y%m%d").to_string().as_str())
        .replace("${ymd}", date.format("%y%m%d").to_string().as_str())
        .replace("${md}", date.format("%m%d").to_string().as_str())
        .replace("${dm}", date.format("%d%m").to_string().as_str())
        .replace("${Yj}", date.format("%Y%j").to_string().as_str())
        .replace("${yj}", date.format("%y%j").to_string().as_str())
}
