use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime};
use std::thread::sleep;
use chrono::{Utc, Local, FixedOffset, Months,  Datelike, Timelike,Days,NaiveTime,NaiveDateTime,NaiveDate};

fn get_runtime() {
    let start = Instant::now();
    sleep(Duration::from_secs(2));
    let duration = start.elapsed();
    dbg!(duration);
}


fn get_runtime2() {
    let start = SystemTime::now();
    sleep(Duration::from_secs(2));
    let end = SystemTime::now();
    dbg!(end.duration_since(start).unwrap());
}


fn chrono_get_time() {
    let standard = Utc::now();
    // 知道当前的时区
    let timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let zone_time = standard.with_timezone(&timezone);
    let local = Local::now();
    dbg!(standard);
    dbg!(local);
    dbg!(zone_time);
}


fn check_add_sub() {
    let local = Local::now();
    let add_day = local.checked_add_days(Days::new(10));
    let add_month = local.checked_add_months(Months::new(1));
    let add_signed = local.checked_add_signed(chrono::Duration::minutes(30));
    let sub_signed = local.checked_sub_signed(chrono::Duration::days(1));
    dbg!(add_day.unwrap());
    dbg!(sub_signed.unwrap());
}

fn get_date_time(){
        let local = Local::now();
        // BCE => BC : 公元前 Before Christ | Before Common Era
        let year_ce = local.year_ce();
        let year = local.year();
        let hour = local.hour();
        let minute = local.minute();
        dbg!(year_ce);
        dbg!(year);
}

fn get_timestamp(){
    let date = NaiveDate::from_ymd_opt(2011, 11, 11).unwrap();
    let date_timestamp = date.and_time(NaiveTime::default());
    dbg!(date_timestamp.timestamp());
}

fn naive_date_time(){
    let now = Local::now().timestamp();
    let date1 = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 9, 12).unwrap(),
        NaiveTime::from_hms_opt(12, 0, 1).unwrap(),
    );
    let date2 = NaiveDate::from_ymd_opt(2023, 9, 12)
        .unwrap()
        .and_hms_opt(12, 1, 1).unwrap();
    let date3 = NaiveDateTime::from_str("2021-12-21T12:00:00").unwrap();
    let date4 = NaiveDateTime::from_timestamp_opt(now, 0).unwrap();
    let fmt = "%Y-%m-%d %H:%M:%S";
    /// %Y : 2023 四位数年份(0000~9999)
    /// %y : 二位数年份(00~99)
    /// %M : 分钟(0~59)
    /// %m : 月份(1~12)
    /// %d : 日(1~31取决于当月)
    /// %H : 24小时数(0~23)
    /// %I : 12小时(1~12)
    /// %S : 秒(0~59)
    let date5 = NaiveDateTime::parse_from_str("2021-12-21 12:00:00", fmt);
    dbg!(date1);
    dbg!(date2);
    dbg!(date3);
    dbg!(date4);
    dbg!(date5.unwrap());
}


fn format_time(){
    /// %Y : 2023 四位数年份(0000~9999)
    /// %y : 二位数年份(00~99)
    /// %M : 分钟(0~59)
    /// %m : 月份(1~12)
    /// %d : 日(1~31取决于当月)
    /// %H : 24小时数(0~23)
    /// %I : 12小时(1~12)
    /// %S : 秒(0~59)
    let local = Local::now();
    let format_time = local.format("%y %m %d => %H:%M:%S");
    dbg!(local);
    dbg!(format_time.to_string());
    dbg!(local.to_rfc2822());
    dbg!(local.to_rfc3339());
}