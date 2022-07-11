use crate::constraints::Constraints;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum ParseState {
    VHEADER,
    VCONSTRAINTS,
    HHEADER,
    HCONSTRAINTS,
}
use ParseState::{HCONSTRAINTS, HHEADER, VCONSTRAINTS, VHEADER};

pub fn parse_file(path: &Path) -> Result<(Constraints, Constraints), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut state = VHEADER;
    let mut v_constraints: Constraints = Constraints::new(0);
    let mut h_constraints: Constraints = Constraints::new(0);

    let (mut vi, mut hi): (usize, usize) = (0, 0);

    for line in reader.lines() {
        let mut line = line.map_err(|e| e.to_string())?;

        match state {
            VHEADER => match line.find("v(") {
                Some(0) => {
                    // Get the number of vertical constraints from string v(<number>):
                    let mut size_str = line.split_off("v(".len());
                    size_str.truncate(size_str.len() - ":)".len());

                    let v_size: usize = size_str.parse::<usize>().map_err(|e| e.to_string())?;

                    v_constraints = Constraints::new(v_size);

                    state = VCONSTRAINTS;
                }
                _ => return Err("Puzzle file has improper format".into()),
            },
            HHEADER => match line.find("h(") {
                Some(0) => {
                    // Get the number of horizontal constraints from string h(<number>):
                    let mut size_str = line.split_off("h(".len());
                    size_str.truncate(size_str.len() - ":)".len());

                    let h_size: usize = size_str.parse::<usize>().map_err(|e| e.to_string())?;

                    h_constraints = Constraints::new(h_size);

                    state = HCONSTRAINTS;
                }
                _ => return Err("Puzzle file has improper format".into()),
            },
            VCONSTRAINTS => {
                if line == "" {
                    state = HHEADER
                } else {
                    let mut constraints: Vec<u32> = Vec::new();
                    for num in line.split_whitespace() {
                        constraints.push(num.parse::<u32>().map_err(|e| e.to_string())?);
                    }
                    v_constraints.set(vi, constraints)?;
                    vi += 1;
                }
            }
            HCONSTRAINTS => {
                let mut constraints: Vec<u32> = Vec::new();
                for num in line.split_whitespace() {
                    constraints.push(num.parse::<u32>().map_err(|e| e.to_string())?);
                }
                h_constraints.set(hi, constraints)?;
                hi += 1;
            }
        }
    }

    Ok((v_constraints, h_constraints))
}
