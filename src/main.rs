#[macro_use]
extern crate peroxide;
use peroxide::prelude::*;

mod board;
use board::*;

fn main() {
    let mut b = Board::new();
    println!("{}", b);
    b.click(0, 0);
    println!("{}", b);
}