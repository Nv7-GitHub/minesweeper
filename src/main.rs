mod board;
use board::*;

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    for r in 0..ROWS {
        for c in 0..COLS {
            board.click(r, c);
            println!("{}", board);
        }
    }
}
