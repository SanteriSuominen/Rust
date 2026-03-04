use chrono::NaiveDate;

use crate::events::{Category, Event};
use crate::providers::EventProvider;

#[derive(Debug, Default)]
pub struct RustHistoryProvider;

impl EventProvider for RustHistoryProvider {
    fn name(&self) -> String {
        String::from("Rust history provider")
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        events.push(Event::new_singular(
            NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
            String::from("Rust 1.92.0 released"),
            Category::from_str("programming/rust"),
        ));
        events.push(Event::new_singular(
            NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
            String::from("Rust 1.0.0 released"),
            Category::new("programming", "rust"),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::MonthDay;

    #[test]
    fn get_events_adds_two_events() {
        let provider = RustHistoryProvider;
        let mut events = Vec::new();

        provider.get_events(&mut events);

        assert_eq!(events.len(), 2);
    }

    #[test]
    fn first_event_has_expected_values() {
        let provider = RustHistoryProvider;
        let mut events = Vec::new();

        provider.get_events(&mut events);

        let first = &events[0];
        assert_eq!(first.description(), "Rust 1.92.0 released");
        assert_eq!(first.category().to_string(), "programming/rust");
        assert_eq!(first.year(), 2025);
        assert_eq!(first.month_day(), MonthDay::new(12, 11));
    }
}
