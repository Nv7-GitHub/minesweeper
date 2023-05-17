#[macro_use]
extern crate peroxide;
use peroxide::prelude::*;

mod board;
use board::*;

fn main() {
    let b = Board::new();
    println!("{}", b);
}