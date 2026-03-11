use crate::events::Event;

pub mod file_provider;
pub use file_provider::{FileEventProvider, ProviderError};

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>) -> Result<(), ProviderError>;
}
