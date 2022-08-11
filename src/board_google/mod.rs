mod detect;
use detect::*;
use enigo::*;
use std::fmt::Display;

pub const COLS: usize = 18;
pub const ROWS: usize = 14;
pub const MINES: usize = 40;
pub const GOOGLE: bool = true;

pub struct Board {
  pub open: [[bool; COLS]; ROWS],
  pub nums: [[u8; COLS]; ROWS],

  start: (i32, i32), // top-left of grid
  sqsize: i32,
  enigo: Enigo,
}

impl Board {
  pub fn new() -> Self {
    let mut v = Self { open: [[false; COLS]; ROWS], nums: [[0; COLS]; ROWS], start: (0, 0), sqsize: 0, enigo: Enigo::new() };
    v.detect();
    v
  }

  fn detect(&mut self) {
    let res = detect();
    for r in 0..ROWS {
      for c in 0..COLS {
        match res.0[r][c] {
          9 => {self.open[r][c] = false},
          _ => {self.nums[r][c] = res.0[r][c]; self.open[r][c] = true;}
        }
      }
    }
    self.start = res.1;
    self.sqsize = res.2;
  }

  fn click_pos(&mut self, row: usize, col: usize) {
    self.enigo.mouse_move_to(self.start.0 + (self.sqsize * col as i32) + (self.sqsize/2), self.start.1 + (self.sqsize * row as i32) + (self.sqsize/2));
    self.enigo.mouse_click(MouseButton::Left);
  }

  pub fn click(&mut self, row: usize, col: usize) -> bool { // Returns whether mine clicked
    // Click
    self.click_pos(row, col);
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Move mouse away to open spot where it won't interfere with number detection
    'outer: for r in 0..ROWS {
      for c in 0..COLS {
        if self.open[r][c] && self.nums[r][c] == 0 {
          self.click_pos(r, c);
          break 'outer;
        }
      }
    }
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Re-detect
    self.detect();

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