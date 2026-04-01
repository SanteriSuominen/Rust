use crate::events::{Category, Event, MonthDay};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFilter {
    month_day: Option<MonthDay>,
    category: Option<Category>,
    text: Option<String>,
}

impl EventFilter {
    pub fn builder() -> FilterBuilder {
        FilterBuilder::new()
    }

    pub fn accepts(&self, event: &Event) -> bool {
        if let Some(month_day) = &self.month_day {
            if event.month_day() != *month_day {
                return false;
            }
        }

        if let Some(category) = &self.category {
            if event.category() != category {
                return false;
            }
        }

        if let Some(text) = &self.text {
            if !event.description().contains(text) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterBuilder {
    month_day: Option<MonthDay>,
    category: Option<Category>,
    text: Option<String>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn month_day(mut self, month_day: MonthDay) -> Self {
        self.month_day = Some(month_day);
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            month_day: self.month_day,
            category: self.category,
            text: self.text,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{Category, Event, EventFilter, MonthDay};

    fn make_event(date: &str, description: &str, category: Category) -> Event {
        let date = NaiveDate::parse_from_str(date, "%F").unwrap();
        Event::new_singular(date, description.to_string(), category)
    }

    #[test]
    fn accepts_all_events_when_no_criteria() {
        let filter = EventFilter::builder().build();
        let event = make_event(
            "2026-03-25",
            "Rust lecture",
            Category::from_primary("school"),
        );

        assert!(filter.accepts(&event));
    }

    #[test]
    fn filters_by_month_day() {
        let filter = EventFilter::builder()
            .month_day(MonthDay::new(3, 25))
            .build();
        let accepted = make_event(
            "2026-03-25",
            "Rust lecture",
            Category::from_primary("school"),
        );
        let rejected = make_event(
            "2026-03-26",
            "Rust lecture",
            Category::from_primary("school"),
        );

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&rejected));
    }

    #[test]
    fn filters_by_category() {
        let filter = EventFilter::builder()
            .category(Category::from_primary("holiday"))
            .build();
        let accepted = make_event(
            "2026-03-25",
            "Independence Day",
            Category::from_primary("holiday"),
        );
        let rejected = make_event("2026-03-25", "Team meeting", Category::from_primary("work"));

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&rejected));
    }

    #[test]
    fn filters_by_text_in_description() {
        let filter = EventFilter::builder().text("Rust").build();
        let accepted = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let rejected = make_event(
            "2026-03-25",
            "Math workshop",
            Category::from_primary("school"),
        );

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&rejected));
    }

    #[test]
    fn filters_by_month_day_and_category() {
        let filter = EventFilter::builder()
            .month_day(MonthDay::new(3, 25))
            .category(Category::from_primary("school"))
            .build();
        let accepted = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_day = make_event(
            "2026-03-26",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_category = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("work"),
        );

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&wrong_day));
        assert!(!filter.accepts(&wrong_category));
    }

    #[test]
    fn filters_by_month_day_and_text() {
        let filter = EventFilter::builder()
            .month_day(MonthDay::new(3, 25))
            .text("Rust")
            .build();
        let accepted = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_day = make_event(
            "2026-03-26",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_text = make_event(
            "2026-03-25",
            "Math workshop",
            Category::from_primary("school"),
        );

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&wrong_day));
        assert!(!filter.accepts(&wrong_text));
    }

    #[test]
    fn filters_by_category_and_text() {
        let filter = EventFilter::builder()
            .category(Category::from_primary("school"))
            .text("Rust")
            .build();
        let accepted = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_category = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("work"),
        );
        let wrong_text = make_event(
            "2026-03-25",
            "Math workshop",
            Category::from_primary("school"),
        );

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&wrong_category));
        assert!(!filter.accepts(&wrong_text));
    }

    #[test]
    fn filters_by_all_three_criteria() {
        let filter = EventFilter::builder()
            .month_day(MonthDay::new(3, 25))
            .category(Category::from_primary("school"))
            .text("Rust")
            .build();
        let accepted = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_day = make_event(
            "2026-03-26",
            "Rust workshop",
            Category::from_primary("school"),
        );
        let wrong_category = make_event(
            "2026-03-25",
            "Rust workshop",
            Category::from_primary("work"),
        );
        let wrong_text = make_event(
            "2026-03-25",
            "Math workshop",
            Category::from_primary("school"),
        );

        assert!(filter.accepts(&accepted));
        assert!(!filter.accepts(&wrong_day));
        assert!(!filter.accepts(&wrong_category));
        assert!(!filter.accepts(&wrong_text));
    }
}
