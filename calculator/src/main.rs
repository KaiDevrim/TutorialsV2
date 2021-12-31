use std::env::{args, Args};
fn main() {
    println!("Hello, world!");
    let mut args: Args = args();
    println!("{:?}", args.nth(0).unwrap());
}
