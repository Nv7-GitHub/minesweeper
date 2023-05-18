/*#[macro_use]
extern crate peroxide;*/

mod board;
use board::*;
mod solver;
use solver::solve;

use std::time::Instant;

fn makegoodboard() -> Board {
    let mut b: Board;
    loop {
        b = Board::new();
        if !b.click(ROWS/2, COLS/2) && b.peek(ROWS/2, COLS/2).unwrap() == 0 {
            break;
        }
    }
    b
}

fn play() -> bool {
    let mut b = makegoodboard(); 
    let mut pos = (ROWS/2, COLS/2);
    println!("{}", b);
    
    let mut res = b.click(pos.0, pos.1);
    while !res {
        println!("{}", b);
        if b.solved() {
            println!("SOLVED!");
            break;
        }
        let start = Instant::now();
        pos = solve(&mut b);
        println!("Solve time: {:.2?}", start.elapsed());
        res = b.click(pos.0, pos.1);
    }
    println!("{}", b);

    return !res; // Returns true if won
}

fn main() {
    while !play() {}
}