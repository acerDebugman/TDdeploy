use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use rhai::ImmutableString;
use dateparser::parse; // 导入 parse 函数
// use chrono::{Local, TimeZone}; // 导入 Local 时区



#[tokio::main]
pub async fn main() -> anyhow::Result<()> {

    let dt = "2025-07-16 17:24:48";
    let dt = format!("{}+08:00", dt);

    let rs = between_time_range(dt.into(), -3600, 60);
    println!("rs: {:?}", rs);

    Ok(())
}

pub async fn test_1() -> anyhow::Result<()> {
    try_parse("2025-06-27 17:24:30");
    println!("***********");


    let date_str1 = "2025-06-27 17:24:30"; // 常见格式
    let date_str2 = "June 27, 2025 5:24:30 PM"; // 另一种常见格式
    let date_str3 = "2025/06/27 17:24:30.123"; // 包含毫秒
    let date_str4 = "昨天 10:00 AM"; // 甚至可以尝试解析相对时间 (英文环境)
    let date_str5 = "2025-06-27T17:24:30Z"; // 已经是RFC3339，也能解析

    let strings_to_parse = vec![
        date_str1,
        date_str2,
        date_str3,
        date_str4,
        date_str5,
    ];

    for s in strings_to_parse {
        println!("--- 尝试解析字符串: \"{}\" ---", s);
        match parse(s) {
            Ok(datetime) => {
                // dateparser 返回的是 DateTime<Utc>，需要转换为本地时区
                let local_datetime = datetime.with_timezone(&Local);
                let rfc3339_local_str = local_datetime.to_rfc3339();
                println!("解析成功！");
                println!("原始 DateTime (UTC): {:?}", datetime);
                println!("转换为本地时区: {:?}", local_datetime);
                println!("RFC 3339 (Local) 格式: {}", rfc3339_local_str);
            }
            Err(e) => {
                println!("解析失败: {}", e);
            }
        }
        println!("");
    }



    let dt = "2025-07-16 16:24:48";
    // let new_dt = chrono::DateTime::parse_from_str(s, fmt)
    let date_str = "2025-06-27 17:24:30";
    let format = "%Y-%m-%d %H:%M:%S"; // 定义输入字符串的格式

    // --- 方法 1: 假定为 UTC 时间 ---
    // 1. 解析为 NaiveDateTime (不带时区)
    let naive_datetime = NaiveDateTime::parse_from_str(date_str, format)
        .expect("无法解析日期时间字符串为 NaiveDateTime");

    // 2. 将 NaiveDateTime 转换为带 UTC 时区的 DateTime
    //    这里的 .and_utc() 假定输入的字符串表示的就是 UTC 时间
    // let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);
    let datetime_utc: DateTime<Local> = Local.from_utc_datetime(&naive_datetime);

    // 3. 格式化为 RFC 3339 字符串
    let rfc3339_utc_str = datetime_utc.to_rfc3339();
    println!("假设为 UTC 时间，RFC 3339 格式: {}", rfc3339_utc_str);
    // 预期输出: 假设为 UTC 时间，RFC 3339 格式: 2025-06-27T17:24:30+00:00 (或Z)


    let rs = between_time_range(dt.into(), -3600, 60);
    println!("rs: {:?}", rs);

    Ok(())
}

pub fn try_parse(s: &str) -> DateTime<Local> {
    // let s = "2025-06-27 17:24:30";
    match parse(s) {
        Ok(datetime) => {
            println!("原始 DateTime (UTC): {:?}", datetime);
            let local_datetime = datetime.with_timezone(&Local);
            let rfc3339_local_str = local_datetime.to_rfc3339();
            println!("转换为本地时区: {:?}", local_datetime);
            println!("RFC 3339 (Local) 格式: {}", rfc3339_local_str);
            local_datetime
        }
        Err(e) => {
            println!("解析失败: {}", e);
            Local::now()
        }
    }
}

pub fn between_time_range(s: ImmutableString, l_sec: i64, r_sec: i64) -> bool {
    println!("s: {:?}, l_sec: {}, r_sec: {}", s, l_sec, r_sec);
    let (now, t) = match chrono::DateTime::parse_from_rfc3339(&s) {
        Ok(dt) => {
            println!("dt: {:?}", dt);
            (
                chrono::Utc::now().with_timezone(&dt.timezone()).timestamp(),
                dt.timestamp(),
            )
        },
        Err(e) => {
            println!("error: {:?}", e);
            (chrono::Local::now().timestamp(), 0)
        },
    };

    let l_timestamp = now + l_sec;
    let r_timestamp = now + r_sec;

    println!("now: {:?}, t: {:?}, l_timestamp: {:?}, r_timestamp: {:?}", now, t, l_timestamp, r_timestamp);

    t > l_timestamp && t < r_timestamp
}