#[macro_use]
extern crate peroxide;

mod board;
use board::*;
mod solver;
use solver::solve;

fn main() {
    let mut b = Board::new();
    let mut pos = (ROWS/2, COLS/2);
    println!("{}", b);
    
    while !b.click(pos.0, pos.1) {
        println!("{}", b);
        if b.solved() {
            break;
        }
        pos = solve(&mut b);
    }
}