<<<<<<< HEAD
use std::fmt;

use chrono::{Datelike, NaiveDate};

#[derive(Debug)]
pub enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub description: String,
    pub category: Category,
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }

    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }

    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.year(),
            self.description,
            self.category
        )
    }
}
=======
use chrono::{Datelike, NaiveDate};
use std::fmt;
>>>>>>> 24dd8ea67d97c113c73747eb2ab7bb906304006e

#[derive(Debug, PartialEq)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }

    pub fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = month_string.parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
}

#[derive(Debug)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }

    pub fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
<<<<<<< HEAD
=======

    pub fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() < 2 {
            Category {
                primary: parts[0].to_string(),
                secondary: None,
            }
        } else {
            Category {
                primary: parts[0].to_string(),
                secondary: Some(parts[1].to_string()),
            }
        }
    }
>>>>>>> 24dd8ea67d97c113c73747eb2ab7bb906304006e
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

<<<<<<< HEAD
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(1 + 1, 2);
    }
}
=======
#[derive(Debug)]
pub enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }

    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }

    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn category(&self) -> &Category {
        &self.category
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            match &self.kind {
                EventKind::Singular(date) => date.year(),
            },
            self.description,
            self.category
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_primary_and_secondary() {
        let cat = Category::new("programming", "rust");

        assert_eq!(cat.primary, "programming");
        assert_eq!(cat.secondary, Some("rust".to_string()));
    }

    #[test]
    fn test_from_primary_only() {
        let cat = Category::from_primary("programming");

        assert_eq!(cat.primary, "programming");
        assert_eq!(cat.secondary, None);
    }

    #[test]
    fn test_from_str_with_both_parts() {
        let cat = Category::from_str("programming/rust");

        assert_eq!(cat.primary, "programming");
        assert_eq!(cat.secondary, Some("rust".to_string()));
    }

    #[test]
    fn test_from_str_primary_only() {
        let cat = Category::from_str("programming");

        assert_eq!(cat.primary, "programming");
        assert_eq!(cat.secondary, None);
    }
}
>>>>>>> 24dd8ea67d97c113c73747eb2ab7bb906304006e
