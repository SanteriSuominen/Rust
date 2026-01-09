mod math;
use math::add;
use math::sub;
fn main() {
    let a: i32 = 2;
    let b: i32 = 1;
    
    println!("a = {a}, b = {b}");

    let result: i32 = add(a, b);
    println!("a + b = {result}");

    let result: i32 = sub(a, b);
    println!("a - b = {result}");
}
