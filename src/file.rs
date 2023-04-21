use chrono::prelude::*;
use crate::*;
use log::*;
use serde::{Serialize, Deserialize};

pub fn read_file(file_name: &str) -> Vec<Item> {
    let content = std::fs::read(file_name).unwrap();
    let content = std::str::from_utf8(&content).unwrap();
    let items: ItemsInDisk = toml::from_str(content).unwrap();
    debug!("read {} items from {}.", items.items.len(), file_name);
    items.items.into_iter().map(|x| x.into()).collect()
}

pub fn write_out(file_name: &str, items: Vec<Item>) {
    debug!("write {} items into {}.", items.len(), file_name);
    let items: Vec<ItemInDisk> = items.into_iter()
        .map(|x| x.into())
        .collect();
    let items = ItemsInDisk {
        items,
    };
    let content = toml::to_string_pretty(&items).unwrap();
    std::fs::write(file_name, &content).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ItemsInDisk {
    items: Vec<ItemInDisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ItemInDisk {
    #[serde(rename = "q")]
    question: String,
    #[serde(rename = "a")]
    answer: String,

    #[serde(rename = "last-remember-time")]
    last_remember_time: Option<toml::value::Datetime>,
    #[serde(rename = "last-check-time")]
    last_check_time: Option<toml::value::Datetime>,
    #[serde(rename = "due-time")]
    due_time: Option<toml::value::Datetime>,
    duration: Option<String>,
}


impl From<ItemInDisk> for Item {
    fn from(value: ItemInDisk) -> Self {
        Self {
            question: value.question,
            answer: value.answer,
            last_remember_time: value.last_remember_time.map(|x| {
                let wrapped: WrapDatetime = x.into();
                wrapped.0
            }),
            last_check_time: value.last_check_time.map(|x| {
                let wrapped: WrapDatetime = x.into();
                wrapped.0
            }),
            due_time: value.due_time.map(|x| {
                let wrapped: WrapDatetime = x.into();
                wrapped.0
            }),
            duration: value.duration.map(|x| {
                let dur: iso8601_duration::Duration = x.parse().unwrap();
                dur.to_chrono().unwrap()
            })
        }
    }
}

impl From<Item> for ItemInDisk {
    fn from(value: Item) -> Self {
        Self {
            question: value.question,
            answer: value.answer,
            last_remember_time: value.last_remember_time.map(|x| WrapDatetime(x).into()),
            last_check_time: value.last_check_time.map(|x| WrapDatetime(x).into()),
            due_time: value.due_time.map(|x| WrapDatetime(x).into()),
            duration: value.duration.map(|x| format!("{}", x)),
        }
    }
}

struct WrapDatetime(UtcTime);

impl From<toml::value::Datetime> for WrapDatetime {
    fn from(value: toml::value::Datetime) -> Self {
        assert!(value.offset.is_some());
        assert_eq!(value.offset.unwrap(), toml::value::Offset::Z);
        let toml_date = value.date.unwrap();
        let toml_time = value.time.unwrap();
        let date = chrono::NaiveDate::from_ymd_opt(
            toml_date.year as i32, toml_date.month as u32, toml_date.day as u32).unwrap();
        let time = chrono::NaiveTime::from_hms_nano_opt(
            toml_time.hour as u32, toml_time.minute as u32, toml_time.second as u32, toml_time.nanosecond as u32).unwrap();
        let datetime = chrono::NaiveDateTime::new(date, time);
        let datetime = UtcTime::from_utc(datetime, Utc);
        Self(datetime)
    }
}

impl From<WrapDatetime> for toml::value::Datetime {
    fn from(value: WrapDatetime) -> Self {
        let date = value.0.date_naive();
        let time = value.0.time();
        toml::value::Datetime {
            date: Some(toml::value::Date {
                year: date.year() as u16,
                month: date.month() as u8,
                day: date.day() as u8,
            }),
            time: Some(toml::value::Time {
                hour: time.hour() as u8,
                minute: time.minute() as u8,
                second: time.second() as u8,
                nanosecond: 0,
            }),
            offset: Some(toml::value::Offset::Z),
        }
    }
}
