use today::events::Event;
use today::providers::rust_history_provider::RustHistoryProvider;
use today::providers::EventProvider;

fn main()
 {
    let mut events: Vec<Event> = Vec::new();

    let provider = RustHistoryProvider;
    provider.get_events(&mut events);

    for event in events {
        println!("{}: {}", event.year(), event.description());
    }
}