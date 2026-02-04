use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Month 
{
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

struct Date 
{
    year: u16,
    month: Month,
    day: u8,
}

impl Date
{
    fn new(year: u16, month: Month, day: u8) -> Self
    {
        Self{year, month, day}
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay
{
    month: Month,
    day: u8,
}

#[derive(Debug)]
struct Category
{
    primary: String,
    secondary: Option<String>,
}

impl Category
{
    fn new(primary: &str, secondary: &str) -> Self
    {
        Self
        {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
        fn from_primary(primary: &str) -> Self 
        {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{} / {}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

struct Event 
{
    date: Date,
    description: String,
    category: Category,
}

impl Event 
{
    fn new(date: Date, description: String, category: Category) -> Self 
    {
        
       Event
       {
            date,
            description,
            category,
       }
    }

    fn month_day(&self) -> MonthDay 
    {
        MonthDay 
        {
            month: self.date.month,
            day: self.date.day,
        }
    }
}

fn main()
{
    let events = [
        Event::new(
            Date::new(2009, Month::January, 15),
            String::from("Chesley Sullenberger lands US Airways Flight 1549 on the Hudson River"),
            Category::new("flight accident", "US Airways"),
        ),
         Event::new(
            Date::new(2009, Month::January, 15),
            String::from("Chesley Sullenberger lands US Airways Flight 1549 on the Hudson River"),
            Category::new("flight accident", "US Airways"),
        ),
         Event::new(
            Date::new(2009, Month::January, 15),
            String::from("Chesley Sullenberger lands US Airways Flight 1549 on the Hudson River"),
            Category::new("flight accident", "US Airways"),
        ),
        Event::new(
            Date::new(1983, Month::January, 16),
            String::from("Turkish Airlines Flight 158 crashes at Ankara Esenboğa Airport"),
            Category::new("flight accident", "Turkish Airlines"),
        ),
        Event::new(
            Date::new(2008, Month::January, 17),
            String::from("British Airways Flight 38 crashes short of the runway at Heathrow Airport"),
            Category::new("flight accident", "British Airways"),
        ),
        Event::new(
            Date::new(1960, Month::January, 18),
            String::from("Capital Airlines Flight 20 crashes into a farm in Charles City County, Virginia"),
            Category::new("flight accident", "Capital Airlines"),
        ),
        Event::new(
            Date::new(2006, Month::January, 19),
            String::from("A Slovak Air Force Antonov An-24 crashes near Hejce, Hungary"),
            Category::new("flight accident", "Slovak Air Force"),
        ),
        Event::new(
            Date::new(1992, Month::January, 20),
            String::from("Air Inter Flight 148, an Airbus A320-111 crashes into a mountain near Strasbourg"),
            Category::new("flight accident", "Air Inter"),
        ),
        Event::new(
            Date::new(1992, Month::January, 20),
            String::from("Air Inter Flight 148, an Airbus A320-111 crashes into a mountain near Strasbourg"),
            Category::new("flight accident", "Air Inter"),
        ),Event::new(
            Date::new(1992, Month::January, 20),
            String::from("Air Inter Flight 148, an Airbus A320-111 crashes into a mountain near Strasbourg"),
            Category::new("flight accident", "Air Inter"),
        ),
        Event::new(
            Date::new(1985, Month::January, 21),
            String::from("Galaxy Airlines Flight 203 crashes near Reno-Tahoe International Airport in Reno, Nevada"),
            Category::new("flight accident", "Galaxy Airlines"),
        ),
         Event::new(
            Date::new(1973, Month::January, 22),
            String::from("A chartered Boeing 707 explodes in flames upon landing at Kano Airport, Nigeria"),
            Category::from_primary("flight accident"),
        ),
         Event::new(
            Date::new(1982, Month::January, 23),
            String::from("World Airways Flight 30 overshoots the runway at Logan International Airport in Boston, Massachusetts"),
            Category::new("flight accident", "World Airways"),
        ),
         Event::new(
            Date::new(1961, Month::January, 24),
            String::from("Goldsboro B-52 crash: A bomber carrying two H-bombs breaks up in mid-air over North Carolina."),
            Category::new("flight accident", "US Air Force"),
        ),
         Event::new(
            Date::new(1990, Month::January, 25),
            String::from("Avianca Flight 052 crashes in Cove Neck, New York"),
            Category::new("flight accident", "Avianca"),
        ),
         Event::new(
            Date::new(2015, Month::January, 26),
            String::from("An aircraft crashes at Los Llanos Air Base in Albacete, Spain"),
            Category::from_primary("flight accident"),
        ),
         Event::new(
            Date::new(1943, Month::January, 27),
            String::from("World War II: The Eighth Air Force sorties ninety-one B-17s and B-24s to attack the U-boat construction yards"),
            Category::new("flight event", "The Eighth Air Force"),
        ),
         Event::new(
            Date::new(2002, Month::January, 28),
            String::from("TAME Flight 120, a Boeing 727-100, crashes in the Andes mountains in southern Colombia"),
            Category::new("flight accident", "TAME"),
        ),
         Event::new(
            Date::new(1973, Month::January, 29),
            String::from("EgyptAir Flight 741 crashes into the Kyrenia Mountains in Cyprus"),
            Category::new("flight accident", "EgyptAir"),
        ),
    ];

    let month_day = MonthDay {month: Month::January, day: 26,};

    print_single_date(month_day, &events);
    print_every_date(&events); 
}

fn print_single_date(month_day: MonthDay, events: &[Event]) 
{
    println!("Events of {:#?} {}\n", month_day.month, month_day.day);

    for event in events
    {
        let current_date: MonthDay = event.month_day();
        let current_day: u8 = current_date.day;
        let current_month: Month = current_date.month;
        

        if month_day == current_date
        {
            println!("{} {:#?} {}", current_day, current_month, event.date.year);
            println!("{}\n", event.description);
            println!("{}", event.category);
        }
    }
}

fn print_every_date(events: &[Event]) 
{
    let mut count = 15;

    for event in events
    {
        if event.date.day == count
        {
            println!("Events of {:#?} {}", event.date.month, event.date.day); 
            
            for event_inner in events
            {
                if event_inner.month_day() == event.month_day()
                {
                    println!("{}: {}", event_inner.date.year, event_inner.description);
                    println!("{}", event_inner.category);
                    println!("");
                }
            }
            println!("------------------------------------------------------------");
            count = count + 1;
        }
    }
}