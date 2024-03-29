use peroxide::prelude::*;
use super::*;

#[derive(Debug)]
struct Equation {
    vars: Vec<usize>,
    res: usize,
}

pub fn solve(board: &mut Board) -> (usize, usize) {
    // Scan board
    let mut vars = Vec::new();
    let mut eqs = Vec::new();
    for r in 0..ROWS {
        for c in 0..COLS {
            if let Some(sum) = board.peek(r, c) {
                if sum == 0 {
                    continue;
                }
                let mut lhs = Vec::new();
                // Search around
                for row in (r as i32)-1..(r as i32)+2 {
                    for col in (c as i32)-1..(c as i32)+2 {
                        if row < 0 || row >= ROWS as i32 {
                            continue;
                        }
                        if col < 0 || col >= COLS as i32 {
                            continue;
                        }
                        if row == r as i32 && col == c as i32 {
                            continue;
                        }
                        if board.peek(row as usize, col as usize).is_none() {
                            let pos = (row as usize, col as usize);
                            if let Some(ind) = vars.iter().position(|a| *a == pos) {
                                lhs.push(ind);
                            } else {
                                lhs.push(vars.len());
                                vars.push(pos);
                            }
                        }
                    }
                }

                eqs.push(Equation{vars: lhs, res: sum});
            }
        }
    }

    // Create matrix
    let mut m = zeros(eqs.len(), vars.len()+1);
    for (i, eq) in eqs.iter().enumerate() {
        for v in eq.vars.iter() {
            m[(i, *v)] = 1.0;
        }
        m[(i, vars.len())] = eq.res as f64;
    }

    m = m.rref(); // Solve
    
    // Search for solutions
    let mut mines = Vec::new();
    for i in 0..eqs.len() {
        let numvars = m.row(i).iter().filter(|x| **x == 1.0).count();
        let numneg = m.row(i).iter().filter(|x| **x < 0.0).count();
        if numvars > 0 && m[(i, vars.len())] == 0.0 && numneg == 0 { // Check for no mine
            let var = m.row(i).iter().position(|x| *x == 1.0).unwrap();
            return vars[var];
        }

        // If mine, add to mines
        if numvars + numneg >= 1 && m[(i, vars.len())] >= 1.0 {
            let var = m.row(i).iter().position(|x| *x == 1.0).unwrap();
            mines.push(var);
        }
    }

    // If unsolvable, try to optimize for least positive vars
    for bad in 0..2 { // First ignore negative variables, then accept them
        for cnt in 2..vars.len() {
            for i in 0..eqs.len() {
                let numvars = m.row(i).iter().filter(|x| **x == 1.0).count();
                let numneg = m.row(i).iter().filter(|x| **x < 0.0).count();
                if numvars + numneg == cnt && (numneg == 0 || bad == 1) {
                    let var = m.row(i).iter().position(|x| *x == 1.0).unwrap();
                    if mines.contains(&var) {
                        continue;
                    }
                    println!("GUESS");
                    return vars[var];
                }
            }
        }
        println!("MOVING TO BAD GUESS")
    }
    
    panic!("UNSOLVABLE");
}