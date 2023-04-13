use chrono::prelude::*;
use clap::Parser;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::Write;
use static_init::dynamic;

type UtcTime = chrono::DateTime<Utc>;

fn main() {
    let args = Args::parse();

    let mut items = read_file(&args.file_name);
    let now = Utc::now();
    let mut selected = out_of_date(&mut items, &now);
    let mut rng = rand::thread_rng();
    selected.shuffle(&mut rng);
    let mut it: Box<dyn Iterator<Item = &mut Item>> = Box::new(selected.into_iter());
    if let Some(n) = args.take {
        it = Box::new(it.take(n));
    }
    for x in it {
        println!("Q: {}", x.question);
        println!("A: {}", x.answer);
        print!("Is it correct? [Y/n] ");
        std::io::stdout().lock().flush().unwrap();
        let mut resp = String::new();
        std::io::stdin().read_line(&mut resp).unwrap();
        match resp.as_str() {
            "Y\n" | "y\n" | "\n" => {
                x.correct(&now);
            }
            _ => {
                x.wrong(&now);
            }
        }
    }
    items.sort_by_key(|x| x.question.clone());
    write_out(&args.file_name, items);
}

fn out_of_date<'a>(items: &'a mut Vec<Item>, now: &DateTime<Utc>) -> Vec<&'a mut Item> {
    items.iter_mut()
        .filter(|x| {
            if let Some(due) = x.due_time {
                due < *now
            } else {
                true
            }
        })
        .collect()
}

/// Pick up something in the memory palace.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the file of a memory palace.
    #[arg()]
    file_name: String,

    /// Take at most `TAKE` things to remember.
    #[arg(long)]
    take: Option<usize>,
}

fn read_file(file_name: &str) -> Vec<Item> {
    let content = std::fs::read(file_name).unwrap();
    let content = std::str::from_utf8(&content).unwrap();
    let items: ItemsInDisk = toml::from_str(content).unwrap();
    items.items.into_iter().map(|x| x.into()).collect()
}

fn write_out(file_name: &str, items: Vec<Item>) {
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

struct Item {
    question: String,
    answer: String,
    last_remember_time: Option<UtcTime>,
    last_check_time: Option<UtcTime>,
    due_time: Option<UtcTime>,
    duration: Option<chrono::Duration>,
}

#[dynamic]
static MAX_DURATION: chrono::Duration = chrono::Duration::days(180);
#[dynamic]
static INIT_DURATION: chrono::Duration = chrono::Duration::hours(8);

impl Item {

    fn correct(&mut self, now: &UtcTime) {
        self.last_remember_time.replace(*now);
        if let Some(ref mut dur) = self.duration {
            *dur = *dur + *dur;
            if *dur > *MAX_DURATION {
                *dur = MAX_DURATION.clone();
            }
        } else {
            self.duration.replace(INIT_DURATION.clone());
        }
        let timeout = self.timeout();
        self.due_time.replace(*now + timeout);
        self.last_check_time.replace(*now);
    }

    fn timeout(&mut self) -> chrono::Duration {
        let int_dur = self.duration.unwrap().num_seconds();
        let mut rng = rand::thread_rng();
        let timeout = rng.gen_range((int_dur/2)..int_dur);
        chrono::Duration::seconds(timeout)
    }

    fn wrong(&mut self, now: &UtcTime) {
        self.duration.replace(INIT_DURATION.clone());
        let timeout = self.timeout();
        self.due_time.replace(*now + timeout);
        self.last_check_time.replace(*now);
    }
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
