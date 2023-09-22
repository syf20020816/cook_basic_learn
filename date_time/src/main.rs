mod lib;

use std::str::FromStr;
// use chrono::{Utc, Local, FixedOffset, Months, Duration, Datelike, Timelike, Days};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Local};

/// %Y : 2023 四位数年份(0000~9999)
/// %y : 二位数年份(00~99)
/// %M : 分钟(0~59)
/// %m : 月份(1~12)
/// %d : 日(1~31取决于当月)
/// %H : 24小时数(0~23)
/// %I : 12小时(1~12)
/// %S : 秒(0~59)
fn main() {
    let local = Local::now();
    let format_time = local.format("%y %m %d => %H:%M:%S");
    dbg!(local);
    dbg!(format_time.to_string());
    dbg!(local.to_rfc2822());
    dbg!(local.to_rfc3339());
}
