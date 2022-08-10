use std::fmt::Display;

use rand::Rng;

pub const COLS: usize = 10;
pub const ROWS: usize = 8;

pub struct Board {
  pub open: [[bool; COLS]; ROWS],
  pub nums: [[u8; COLS]; ROWS],
  pub flags: [[bool; COLS]; ROWS],

  // These will be removed and the board will be updated through google minesweeper instead
  internalnums: [[u8; COLS]; ROWS],
  internalmines: [[bool; COLS]; ROWS]
}

impl Board {
  pub fn new() -> Self {
    let mut v = Self { open: [[false; COLS]; ROWS], nums: [[0; COLS]; ROWS], flags: [[false; COLS]; ROWS], internalnums: [[0; COLS]; ROWS], internalmines: [[false; COLS]; ROWS] };
    v.geninternal();
    v
  }

  pub fn click(&mut self, row: usize, col: usize) {
    // This code will be replaced by screen clicking code & image processing to find new state of board after click
    self.open[row][col] = true;
    if self.internalmines[row][col] {
      panic!("mine clicked!")
    }
    self.nums[row][col] = self.internalnums[row][col];

    // 0, open areas around too
    if self.nums[row][col] == 0 {
      for roff in -1..2 {
        for coff in -1..2 {
          let rv = row as i32 + roff;
          let cv = col as i32 + coff;
          if rv < 0 || rv as usize >= ROWS || cv < 0 || cv as usize >= COLS {
            continue;
          }

          // Mine, increment count
          if !self.open[rv as usize][cv as usize] {
            self.click(rv as usize, cv as usize);
          }
        }
      }
    }
  }

  pub fn flag(&mut self, row: usize, col: usize) {
    // Screen clicking will be added to this
    self.flags[row][col] = true;
  }

  fn geninternal(&mut self) {
    // Place mines
    let mut rng = rand::thread_rng();
    const MINES: usize = 10;
    let mut mines = 0;
    'outer: for r in 0..ROWS {
      for c in 0..COLS {
        let v: bool = rng.gen_bool(MINES as f64 / (COLS * ROWS) as f64);
        if v {
          self.internalmines[r][c] = true;
          mines += 1;
          if mines >= MINES {
            break 'outer;
          }
        }
      }
    }

    // Fill out internalnums
    for r in 0..ROWS {
      for c in 0..COLS {
        // Look around
        for roff in -1..2 {
          for coff in -1..2 {
            let rv = r as i32 + roff;
            let cv = c as i32 + coff;
            if rv < 0 || rv as usize >= ROWS || cv < 0 || cv as usize >= COLS {
              continue;
            }

            // Mine, increment count
            if self.internalmines[rv as usize][cv as usize] {
              self.internalnums[r][c] += 1;
            }
          }
        }
      }
    }
  }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      for r in 0..ROWS {
        for c in 0..COLS {
          if self.open[r][c] {
            write!(f, "{}", self.nums[r][c])?;
          } else {
            write!(f, "█")?;
          }
        }
        write!(f, "\n")?;
      }
      Ok(())
    }
}