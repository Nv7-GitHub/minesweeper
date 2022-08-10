mod board;
use board::*;

mod solve;
use solve::*;

fn main() {
    let mut board: Board;

    'makeboard: loop {
        board = Board::new();

        // Open up board
        let mut clicks = 0;
        for r in 0..ROWS {
            for c in 0..COLS {
                if board.click(r, c) {
                    continue 'makeboard; // Clicked a mine, try another board
                }
                clicks += 1;
                
                // Count open
                let mut open = 0;
                for r in 0..ROWS {
                    for c in 0..COLS {
                        if board.open[r][c] {
                            open += 1;
                        }
                    }
                }
                if open > clicks { // Board opened up
                    break 'makeboard;
                }
            }
        }
    }

    // Solve
    println!("{}", board);
    while board.unopened() != MINES {
        // Progress
        let unopen = board.unopened();
        println!("{} Remaining ({:.2}%)", unopen-MINES, (ROWS*COLS - unopen) as f32/(ROWS*COLS - MINES) as f32 * 100.0);

        // Iteration
        let pos = solve_iter(&board);

        // Click and check for mine
        if board.click(pos.0, pos.1) { // Clicked on a mine
            println!("\n{}", board);
            println!("MINE CLICKED");
            break;
        };
    }
    
    // Success
    if board.unopened() == MINES {
        println!("\n{}", board);
        println!("SOLVED")
    }
}
