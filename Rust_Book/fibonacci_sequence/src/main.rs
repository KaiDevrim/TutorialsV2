use std::io;
fn main() {
    let mut x: i128 = 1;
    let mut y: i128 = 1;
    let mut input = String::new();
    let mut counter: i32 = 0;
    println!("Input how many sequences you want.");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let count: i32 = input.trim().parse().expect("Not a number");
    while counter != count {
        counter += 1;
        x += y;
        y += x;
        println!("{}: \n x: {} \n y: {}",counter,x,y);
    }
}
