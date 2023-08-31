mod site;
mod object;


pub use async_trait::async_trait;
pub use object::*;
pub use site::*;
use chrono::Datelike;
use serde::*;
use static_init::dynamic;
use reqwest::Client;


/// static http client that will be used in the whole crate to offer performance and enable request pipelining.
#[dynamic]
static CLIENT: Client = Client::new();



/// convert a day or month int number into a two digit string.
fn str_from_number(number: u32) -> String {
    if number > 9 {
        return number.to_string()
    }
    String::from("0") + &number.to_string()
}

/// getting the current collecting date. if 8:00 am UTC is not reached return yesterdays date else return todays date.
pub fn collecting_date() -> String {
    let mut time = chrono::offset::Utc::now();
    if time.time() < chrono::naive::NaiveTime::from_hms_opt(8,0,0).unwrap() {
        time-=chrono::Duration::days(1);
    }
    let (year, month, day) = (str_from_number(time.year() as u32), str_from_number(time.month()), str_from_number(time.day()));
    month+"_"+&day+"_"+&year
}

/// split a vec of elements into a vec of vec of elements divided by the parameter provided as the limit.
pub fn split<T, I: Iterator<Item = T>>(iter: I, limit: usize) -> Vec<Vec<T>> {
    let mut iter = iter.peekable();
    let mut rounds = Vec::new();
    while let Some(_) = iter.peek() {
        let mut round = Vec::new();
        for _ in 0..limit {
            match iter.next() {
                Some(item) => round.push(item),
                None => break
            }
        }
        rounds.push(round)
    }
    rounds
}