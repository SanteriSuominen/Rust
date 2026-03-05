use crate::events::Event;

pub mod rust_history_provider;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}