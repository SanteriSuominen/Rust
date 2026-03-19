use crate::events::{Category, Event};
use chrono::{Local, NaiveDate};

pub mod textfile;
pub mod csvfile;
pub mod sqlite;
pub mod web;
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}

pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();

        let test_event_1 = Event::new_singular(
            today,
            String::from("Test event 1 for today"),
            Category::from_primary("test"),
        );
        events.push(test_event_1);

        let test_event_2 = Event::new_singular(
            today,
            String::from("Test event 2 for today"),
            Category::from_primary("test"),
        );
        events.push(test_event_2);
    }
}