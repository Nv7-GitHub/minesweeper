mod detect;
use detect::*;
use std::fmt::Display;

pub const COLS: usize = 10;
pub const ROWS: usize = 8;
pub const MINES: usize = 10;
pub const GOOGLE: bool = true;

pub struct Board {
  pub open: [[bool; COLS]; ROWS],
  pub nums: [[u8; COLS]; ROWS],

  start: (i32, i32), // top-left of grid
  sqsize: i32,
}

impl Board {
  pub fn new() -> Self {
    let mut v = Self { open: [[false; COLS]; ROWS], nums: [[0; COLS]; ROWS], start: (0, 0), sqsize: 0 };
    v.detect();
    v
  }

  fn detect(&mut self) {
    let res = detect();
    for r in 0..ROWS {
      for c in 0..COLS {
        match res.0[r][c] {
          9 => {self.open[r][c] = false},
          _ => {self.nums[r][c] = res.0[r][c]}
        }
      }
    }
    self.start = res.1;
    self.sqsize = res.2;
  }

  pub fn click(&mut self, row: usize, col: usize) -> bool { // Returns whether mine clicked
    todo!("unimplemented");

    false
  }

  pub fn unopened(&self) -> usize {
    let mut unopen = 0;
    for r in 0..ROWS {
      for c in 0..COLS {
        if !self.open[r][c] {
          unopen += 1;
        }
      }
    }

    unopen
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