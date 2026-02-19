use std::env;

fn main() 
{
    unsafe
    {
        env::set_var("KT4TVAR", "Hello_World");
    }

    let value = env::var("KT4TVAR").unwrap();
    println!("{value}");

    unsafe 
    {
        env::remove_var("KT4TVAR");
    }
}
