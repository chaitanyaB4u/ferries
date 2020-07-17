use chrono::{NaiveDateTime,Duration,Utc,Timelike};
use std::ops::Sub;
use uuid::Uuid;

const DATE_TIME_PATTERN: &'static str = "%Y-%m-%dT%H:%M:%SZ";

pub fn as_date(date_str: &str) -> NaiveDateTime {

    let parse_from_str = NaiveDateTime::parse_from_str;
    let given_date = parse_from_str(date_str, DATE_TIME_PATTERN).unwrap_or(Utc::now().naive_utc());
    
    strip_seconds(given_date)
}

pub fn strip_seconds(given_date: NaiveDateTime) -> NaiveDateTime {
    let second = Duration::seconds(given_date.second() as i64);
    given_date.sub(second)
}

pub fn is_valid_date(date_str: &str) -> bool {
    let parse_from_str = NaiveDateTime::parse_from_str;
    let given_date = parse_from_str(date_str, DATE_TIME_PATTERN);

    given_date.is_ok()
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn is_past_date(date: NaiveDateTime) -> bool {
    strip_seconds(date) < strip_seconds(now()) 
}


pub fn fuzzy_id() -> String {

    let uuid = Uuid::new_v4();
    let hype = uuid.to_hyphenated().to_string();

    hype.clone()
}

pub fn concat(str1: &str, str2: &str) -> String {
    format!("{} and {}",str1,str2)
}