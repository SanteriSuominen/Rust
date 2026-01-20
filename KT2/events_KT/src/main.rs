fn main() {
    let flight_accident_events= [
        (2009_01_15, "Chesley Sullenberger lands US Airways Flight 1549 on the Hudson River"),
        (1983_01_16, "Turkish Airlines Flight 158 crashes at Ankara Esenboğa Airport"),
        (2008_01_17, "British Airways Flight 38 crashes short of the runway at Heathrow Airport"),
        (1960_01_18, "Capital Airlines Flight 20 crashes into a farm in Charles City County, Virginia"),
        (1995_01_19, "Bristow Helicopters Flight 56C was struck by lightning while carrying oil workers from Aberdeen"),
        (1992_01_20, "Air Inter Flight 148, an Airbus A320-111 crashes into a mountain near Strasbourg"),
        (1985_01_21, "Galaxy Airlines Flight 203 crashes near Reno-Tahoe International Airport in Reno, Nevada"),
        (1973_01_22, "A chartered Boeing 707 explodes in flames upon landing at Kano Airport, Nigeria"),
        (2002_11_11, "syntymäpäivä"),
    ];

    check_events(&flight_accident_events);
    print_all_events(&flight_accident_events);
}

fn check_events(events: &[(i32, &str)]) // ottaa vastaan taulun monikoita ja tulostaa today_stringiä vastaavat eventit
{
    let today:i32 = 2026_01_16;
    let today_str = &today.to_string()[4..8];
    
    for event in events
    {
        let date = &event.0.to_string()[4..8];

        if date == today_str
        {
            println!("{}|{}", event.0, event.1);
        }
    }
}

fn get_date(slice: &str) -> i32 // ottaa vastaan slicen ja muuttaa intiksi, implementoitu putsaamaan koodia
{
    let string_to_convert = slice.to_string();
    return string_to_convert.parse().unwrap();
}

fn print_all_events(events: &[(i32, &str)]) // tulostaa kaikki eventit joka päivältä
{
    println!(""); 
    let beginning: i32 = get_date(&events.first().unwrap().0.to_string()[4..8]); // haetaan ensimmäinen ja viimeinen päivämäärä month,day
    let end: i32 = get_date(&events.last().unwrap().0.to_string()[4..8]);        // näin siksi ettei tarvitse muuttaa koodia mikäli lista muuttuu

    for date in beginning..(end + 1)    // hyödynnetään tiedossa olevat parametrit eli listan alku ja loppu päivämäärät
    {
        for event in events
        {
            if date == get_date(&event.0.to_string()[4..8]) // ei tulosteta tyhjiä päiviä
            {                                                                                
                let date_string = date.to_string();             // kaunistellaan headeria
                let mut day_month = ("day", "month");
                
                if date_string.len() == 4
                {
                    day_month.0 = &date_string[2..4];
                    day_month.1 = &date_string[0..2];
                }

                else 
                {
                    day_month.0 = &date_string[1..3];
                    day_month.1 = &date_string[0..1];
                }

                println!("Events of {}.{}: ",day_month.0, day_month.1);
           

                for event in events             // tulosta kaikki päivän eventit
                {
                    let eventsdate: i32 = get_date(&event.0.to_string()[4..8]);

                    if date == eventsdate
                    {
                        let year = &event.0.to_string()[0..4];
                        println!("{}| {}", year, event.1);
                    } 
                }
                println!(""); 
                break;
            }
        }
    }
}