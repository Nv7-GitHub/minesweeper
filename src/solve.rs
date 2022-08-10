use super::*;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Pos(usize, usize); // row, col

// lhs ids added up = res
#[derive(Debug)]
struct Constraint {
  lhs: Vec<usize>,
  res: u8,
}

#[derive(Debug)]
struct Constraints {
  constraints: Vec<Constraint>,
  pos: Vec<Pos>, // map[id]<row, col>
  results: Vec<usize>, // map[id]chance_mine
}

// Calculating constraints
impl From<&Board> for Constraints {
  fn from(b: &Board) -> Self {
    let mut sel = Self{constraints: Vec::new(), pos: Vec::new(), results: Vec::new()};

    for r in 0..ROWS {
      for c in 0..COLS {
        if b.open[r][c] && b.nums[r][c] > 0 {
          let mut ids = Vec::new();
            
          // Go through neighbors, find closed spots, build constraint
          for roff in -1..2 {
            for coff in -1..2 {
              let rv = r as i32 + roff;
              let cv = c as i32 + coff;
              if rv < 0 || rv as usize >= ROWS || cv < 0 || cv as usize >= COLS {
                continue;
              }
  
              if !b.open[rv as usize][cv as usize] {
                let pos = sel.pos.iter().position(|&v| {v == Pos(rv as usize, cv as usize)});
                if let Some(ind) = pos {
                  // Already in it, just add to constraint
                  ids.push(ind);
                } else {
                  // Add to ids if not in it
                  ids.push(sel.pos.len());
                  sel.pos.push(Pos(rv as usize, cv as usize));
                }
              }
            }
          }

          // Add constraint
          if ids.len() > 0 {
            ids.sort();
            sel.constraints.push(Constraint { lhs: ids, res: b.nums[r][c] })
          }
        }
      }
    }

    // Make results
    sel.results = vec![0; sel.pos.len()];

    sel
  }
}

// Solving using recursive backtracking
impl Constraints {
  fn check_constraints(&self, sol: &Vec<bool>, ind: usize, val: bool) -> bool {
    for c in self.constraints.iter() {
      if c.lhs.contains(&ind) { // Constraint affected by this
        // Calculate sum
        let mut sum = 0;
        for ind in c.lhs.iter() {
          if sol[*ind] {
            sum += 1;
          }
        }
        if val {
          sum += 1;
        }
      
        // If this is last and the sum is below what it should be, this is breaking constraint
        if *c.lhs.last().unwrap() == ind && sum < c.res {
          return false;
        }
        // If the sum is greater than what it should be, then breaking constraint
        if sum > c.res {
          return false;
        }
      }
    }

    true
  }

  fn backtrack(&mut self, sol: &mut Vec<bool>, ind: usize) {
    if ind >= sol.len() { // it worked, update chances
      for (i, v) in sol.into_iter().enumerate() {
        if *v {
          self.results[i] += 1;
        }
      }
      return;
    }

    // Try without this as a mine
    if self.check_constraints(sol, ind, false) {
      let prev = sol[ind];
      sol[ind] = false;
      self.backtrack(sol, ind + 1);
      sol[ind] = prev; // Revert to what it was
    }

    // Try with this as a mine
    if self.check_constraints(sol, ind, true) {
      let prev = sol[ind];
      sol[ind] = true;
      self.backtrack(sol, ind + 1);
      sol[ind] = prev; // Revert to what it was
    }
  }
}

pub fn solve_iter(b: &Board) -> (usize, usize) { // Returns location to click
  let mut c = Constraints::from(b);

  // Backtrack
  let mut sol = vec![false; c.results.len()];
  c.backtrack(&mut sol, 0);

  // Return location with lowest value
  let mut ind = 0;
  for (i, res) in c.results.iter().enumerate() {
    if *res < c.results[ind] {
      ind = i;
    }
  }
  let p = c.pos[ind];

  return (p.0, p.1)
}