use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use sqlite::Connection;
use sqlite::State;

use crate::events::{Category, Event};
use crate::EventProvider;

pub struct SQLiteProvider {
    name: String,
    path: PathBuf,
}

impl SQLiteProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self { 
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }

    fn get_categories(&self, connection: &Connection) -> HashMap<i64, Category> {
        let mut category_map: HashMap<i64, Category> = HashMap::new();
        let category_query = "SELECT category_id, primary_name, secondary_name FROM category";
        let mut statement = connection.prepare(category_query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let primary = statement.read::<String, _>("primary_name").unwrap();
            let secondary = statement.read::<Option<String>, _>("secondary_name").unwrap();
            let category = match secondary {
                Some(sec) => Category::new(&primary, &sec),
                None => Category::from_primary(&primary),
            };
            category_map.insert(category_id, category);
        }
        category_map
    }
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let connection = Connection::open(self.path.clone()).unwrap();
        let category_map = self.get_categories(&connection);
        let event_query: String =
            "SELECT event_date, event_description, category_id FROM event".to_string();
        let mut statement = connection.prepare(event_query).unwrap();

        while let Ok(State::Row) = statement.next() {
            let date_string = statement.read::<String, _>("event_date").unwrap();
            let date = NaiveDate::parse_from_str(&date_string, "%F").unwrap();
            let description = statement.read::<String, _>("event_description").unwrap();
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let category = category_map.get(&category_id).unwrap();

            events.push(Event::new_singular(date, description.to_string(), category.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::events::MonthDay;
    use crate::providers::EventProvider;

    use super::SQLiteProvider;

    #[test]
    #[ignore = "manual smoke test for local SQLite DB"]
    fn print_events_for_19_3_from_sqlite() {
        let db_path = if let Ok(path) = std::env::var("SQL_HISTORY_DB") {
            PathBuf::from(path)
        } else {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home)
                .join(".config")
                .join("today")
                .join("SQL_history.sqlite3")
        };

        assert!(
            db_path.exists(),
            "SQLite DB not found at '{}'. Set SQL_HISTORY_DB to your DB path.",
            db_path.display()
        );

        let provider = SQLiteProvider::new("sqlite-test", &db_path);
        let mut events = Vec::new();
        provider.get_events(&mut events);

        let target = MonthDay::new(3, 19);
        let matching: Vec<_> = events
            .iter()
            .filter(|event| event.month_day() == target)
            .collect();

        assert!(
            !matching.is_empty(),
            "No 19.3 events found in '{}'.",
            db_path.display()
        );

        for event in matching {
            println!("{}", event);
        }
    }
}