fn main() {
    let mut y: i128 = 0;
    let mut x: i128 = 1;

    for i in 0..100 {
        println!("{}.{}", i + 1, y);
        let sum: i128 = x + y;
        y = x;
        x = sum;
    }
}
