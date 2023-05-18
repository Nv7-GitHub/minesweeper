use super::*;
use rand::Rng; 

pub const ROWS: usize = 8;
pub const COLS: usize = 10;
pub const MINES: usize = 10;

#[derive(Default)]
pub struct Board {
    nums: [[usize; COLS]; ROWS],
    mines: [[bool; COLS]; ROWS],
    open: [[bool; COLS]; ROWS],
}

impl Board {
    pub fn new() -> Self {
        // Place mines
        let mut b = Self::default();
        let mut rng = rand::thread_rng();
        for _ in 0..MINES { // TODO: Don't allow mines to be on top of each other
            let r = rng.gen_range(0..ROWS);
            let c = rng.gen_range(0..COLS);
            b.mines[r][c] = true;

            // Update count
            for row in (r as i32)-1..(r as i32)+2 {
                for col in (c as i32)-1..(c as i32)+2 {
                    if row < 0 || row >= ROWS as i32 {
                        continue;
                    }
                    if col < 0 || col >= COLS as i32 {
                        continue;
                    }
                    b.nums[row as usize][col as usize] += 1;
                }
            }
        }
        b
    }

    pub fn click(&mut self, r: usize, c: usize) -> bool {
        if self.open[r][c] {return self.mines[r][c]};
        self.open[r][c] = true;
        if self.mines[r][c] { // Its a mine!
            return true;
        }
        if self.nums[r][c] == 0 { // Flood open
            for row in (r as i32)-1..(r as i32)+2 {
                for col in (c as i32)-1..(c as i32)+2 {
                    if row < 0 || row >= ROWS as i32 {
                        continue;
                    }
                    if col < 0 || col >= COLS as i32 {
                        continue;
                    }
                    if row == r as i32 && col == c as i32{
                        continue;
                    }
                    self.click(row as usize, col as usize);
                }
            }
        }
        false
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in 0..ROWS {
            for c in 0..COLS {
                if self.open[r][c] {
                    if self.mines[r][c] {
                        write!(f, "M")?
                    } else {
                        if self.nums[r][c] > 0 {
                            write!(f, "{}", self.nums[r][c])?;
                        } else {
                            write!(f, " ")?;
                        }
                    }
                } else {
                    write!(f, "█")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}