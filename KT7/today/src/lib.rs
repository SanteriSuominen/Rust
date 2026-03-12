<<<<<<< HEAD
use std::error::Error;

mod birthday;
mod events;
mod providers;

use birthday::handle_birthday;
use chrono::{Datelike, Local, NaiveDate};
use events::{Category, Event, MonthDay};
use providers::{EventProvider, SimpleProvider};

pub fn run() -> Result<(), Box<dyn Error>> {
    handle_birthday();

    let mut events: Vec<Event> = Vec::new();
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        Category::new("programming", "rust"),
    ));
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        Category::new("programming", "rust"),
    ));

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    let simple_provider = SimpleProvider::new("simppeli");
    simple_provider.get_events(&mut events);

    for event in events {
        if today_month_day == event.month_day() {
            println!("{}", event);
        }
    }

    Ok(())
}
=======
pub mod config;
pub mod events;
pub mod providers;
>>>>>>> 24dd8ea67d97c113c73747eb2ab7bb906304006e
