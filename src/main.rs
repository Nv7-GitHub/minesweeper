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
    
    let mut res = b.click(pos.0, pos.1);
    while !res {
        println!("{}", b);
        if b.solved() {
            println!("SOLVED!");
            break;
        }
        pos = solve(&mut b);
        res = b.click(pos.0, pos.1);
    }
    println!("{}", b);

    if res {
        println!("RIP :(");
    }
}