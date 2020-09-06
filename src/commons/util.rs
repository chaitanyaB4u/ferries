use chrono::{Duration, NaiveDateTime, Timelike, Utc};
use sodiumoxide::crypto::pwhash::argon2id13;
use std::ops::Sub;
use uuid::Uuid;

const DATE_TIME_PATTERN: &'static str = "%Y-%m-%dT%H:%M:%SZ";

pub const MEMBER: &'static str = "member";
pub const COACH: &'static str = "coach";

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

pub fn is_in_past(given_date: NaiveDateTime) -> bool {
    let date = given_date.date();
    let now_date = now().date();

    date < now_date
}

pub fn fuzzy_id() -> String {
    let uuid = Uuid::new_v4();
    let hype = uuid.to_hyphenated().to_string();

    hype.clone()
}

pub fn concat(str1: &str, str2: &str) -> String {
    format!("{} and {}", str1, str2)
}

pub fn hash(password: &str) -> String {
    sodiumoxide::init().unwrap();

    let hashed_password = argon2id13::pwhash(
        password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    let text_hash = std::str::from_utf8(&hashed_password.0).unwrap().to_string();

    text_hash
}

/**
 * 
 * 1. Create an array of length 128 and stuff that with 0 (unsigned byte)
 * 2. iterate the given slice and replace the array with the respective value.
 *
 */
fn as_byte_array(slice: &str) ->[u8;128] {

    let mut arr  = [0u8;128];

    slice.as_bytes().iter().enumerate().for_each(|(i, val)| {
        arr[i] = val.clone();
    });

    arr
}

pub fn is_equal(hashed_password: &str, given_password: &str) -> bool {
   
    sodiumoxide::init().unwrap();

    let hashed_bytes = as_byte_array(hashed_password);

    let mut status = false;

    if let Some(hash) = argon2id13::HashedPassword::from_slice(&hashed_bytes) {
        status = argon2id13::pwhash_verify(&hash,given_password.as_bytes());
    }

    status
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_in_past() {
        let start_time = "2020-08-27T06:53:09Z";
        assert_eq!(true,is_in_past(as_date(start_time)));
    }

    #[test]
    fn should_hash_and_verify_hashed_password() {
        let hashed = hash("abcdefghijklmnopqrstuvwxyz");

        assert_eq!(false,is_equal(hashed.as_str(),"harini"));
        assert_eq!(true, is_equal(hashed.as_str(), "abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(false,is_equal(hashed.as_str(), "abcdefghij lmnopqrstuvwxyz"));
    }
}
