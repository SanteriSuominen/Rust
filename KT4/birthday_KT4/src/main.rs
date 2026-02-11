use std::env;
use std::process;
use chrono::{NaiveDate, Datelike, Local}; 

fn main() {
    let birthday = handle_birthdate();
    let system_time = get_system_date();
    handle_results(birthday, system_time);
}

fn handle_birthdate() -> NaiveDate 
{
    // onko olemassa
    if env::var("BIRTHDATE").is_err() 
    {
        eprintln!("BIRTHDATE-ympäristömuuttujaa ei ole asetettu.");
        process::exit(1);
    }

    // jos on niin haetaan
    let birthdate_str = env::var("BIRTHDATE").unwrap();

    // format ja palauta
    let birthdate = match NaiveDate::parse_from_str(&birthdate_str, "%Y-%m-%d") 
    {
        Ok(date) => date,
        Err(_) => 
        {
            eprintln!("BIRTHDATE ei ole kelvollinen ISO 8601 -päivämäärä (YYYY-MM-DD).");
            process::exit(1);
        }
    };

    return birthdate;
}

fn get_system_date() -> NaiveDate
{
    let now = Local::now();
    let system_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()) // from_ymd_opt koska cargo ilmoitti että se on uudempi versio jossa on virhe käsittely
        .expect("Virheellinen päivämäärä!");

    return system_date;
}   

fn handle_results(birthday:NaiveDate, system_time: NaiveDate)
{
    let difference = system_time.signed_duration_since(birthday);
    let age_in_days: i64 =  difference.num_days();
    
    
    let mut message = match age_in_days 
    {
        i64::MIN..=-1 => "Are you from the future?",
        0 =>             "Happy birthday! Looks like you're new here.",
        _ =>             "",
    };

    if age_in_days % 1000 == 0 && age_in_days != 0
    {
        message = "That's a nice, round number!";
    }

    print!("You are {} days old.", age_in_days);
    print!(" {}", message);

    if age_in_days != 0 && birthday.day() == system_time.day() && birthday.month() == system_time.month()
    {
        println!("\nAlso happy birthday!");
    }

}
