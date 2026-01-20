fn main() 
{
    let events = [
        (1996_01_23, "JDK 1.0 released"),
        (2008_12_03, "Python 3.0 released"),
        (2015_05_15, "Rust 1.0.0 released"),
        (2025_09_16, "Java 25 released"),
        (2025_10_07, "python 3.14 released"),
        (2025_12_11, "Rust 3.92.0 released"),
    ];
    
    println!("{} events", events.len());
    println!("{:#?}", events);

    let mut index = 0;

    for (index, event) in events.iter().enumerate()
    {
        println!("{}: {}", event.0, event.1);
    }

    for event in events {
        println!("{}: {}", event.0, event.1)
    }
    
    while index < events.len()
    {
        println!("[{}] {}: {}", index + 1, events[index].0, events[index].1);
        index += 1;
    }
}