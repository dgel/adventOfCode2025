use std::collections::{HashSet, VecDeque};

use advent_of_code::{Mat, Rational, ZeroExt};
use chumsky::prelude::*;
use chumsky::text::{digits, newline};

use smallvec::SmallVec;
use std::fmt;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    target_lights: u64,
    buttons: Vec<SmallVec<[u16; 8]>>,
    joltages: Vec<u16>,
}

impl fmt::Display for Machine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "target: {:#b}, buttons: ", self.target_lights)?;
        for button in &self.buttons {
            write!(f, "(")?;
            if !button.is_empty() {
                write!(f, "{}", button[0])?;
                for num in button.iter().skip(1) {
                    write!(f, "{}", num)?
                }
            }
            write!(f, ")")?;
        }
        write!(f, ", joltages:")?;
        for joltage in &self.joltages {
            write!(f, " {}", joltage)?;
        }
        fmt::Result::Ok(())
    }
}

fn parse(input: &str) -> Option<Vec<Machine>> {
    let num_u32 = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let num_u16 = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };

    let target_lights = choice((just('.'), just('#')))
        .repeated()
        .to_slice()
        .delimited_by(just('['), just(']'))
        .map(|s: &str| {
            s.bytes().enumerate().fold(
                0,
                |acc, (i, byte)| if byte == b'#' { acc | (1 << i) } else { acc },
            )
        });

    let button = num_u16()
        .separated_by(just(','))
        .collect()
        .map(|v| SmallVec::from_vec(v))
        .delimited_by(just('('), just(')'));
    let buttons = button.separated_by(just(' ')).allow_trailing().collect();

    let joltages = num_u32()
        .separated_by(just(','))
        .collect()
        .delimited_by(just('{'), just('}'));

    let machine = target_lights
        .then_ignore(just(' '))
        .then(buttons)
        .then(joltages)
        .map(|((target_lights, buttons), joltages)| Machine {
            target_lights,
            buttons,
            joltages,
        });

    let machines = machine.separated_by(newline()).allow_trailing().collect();

    match machines.parse(input).into_result() {
        Ok(machines) => Some(machines),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

fn solve_machine(machine: &Machine) -> u64 {
    if machine.target_lights == 0 {
        return 0;
    }
    let mut seen_lights = HashSet::new();
    seen_lights.insert(0u64);

    let mut queue = VecDeque::from([(0, 0)]);

    let buttons: Vec<u64> = machine
        .buttons
        .iter()
        .map(|button| button.iter().fold(0, |acc, num| acc | (1 << num)))
        .collect();

    // breadth-first search, so will find a solution before any button is pushed twice
    while let Some((lights, buttons_pushed)) = queue.pop_front() {
        let new_buttons_pushed = buttons_pushed + 1;
        for button in &buttons {
            let new_lights = lights ^ button;
            if new_lights == machine.target_lights {
                return new_buttons_pushed;
            }
            if seen_lights.insert(new_lights) {
                queue.push_back((new_lights, new_buttons_pushed));
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u64> {
    if let Some(machines) = parse(input) {
        let mut num_buttons_pushed = 0;
        for machine in machines {
            let result = solve_machine(&machine);
            num_buttons_pushed += result;
        }
        Some(num_buttons_pushed)
    } else {
        None
    }
}

fn is_solvable<T: ZeroExt>(mat: &Mat<T>) -> bool {
    for row in 0..mat.rows() {
        let row = mat.row(row);
        if let Some((last, rest)) = row.split_last()
            && !last.is_zero()
            && rest.iter().all(|v| v.is_zero())
        {
            return false;
        }
    }
    true
}

fn get_free_vars<T: From<i32> + Eq + ZeroExt>(mat: &Mat<T>) -> Vec<usize> {
    let mut free_vars = Vec::new();
    for col in 0..mat.cols() - 1 {
        let mut is_free_var = true;
        for row in 0..mat.rows() {
            let val = &mat[(row, col)];
            if !val.is_zero() {
                if *val == 1i32.into() {
                    if !is_free_var {
                        is_free_var = true;
                        break;
                    }
                    is_free_var = false;
                } else {
                    is_free_var = true;
                    break;
                }
            }
        }
        if is_free_var {
            free_vars.push(col);
        }
    }
    free_vars
}

fn solve_machine_gaussian_search(
    mat: &Mat<Rational>,
    free_vars: &[usize],
    max_vals: &[u64],
) -> u64 {
    let mut state: Vec<_> = free_vars
        .iter()
        .map(|&idx| {
            (
                idx,
                0u64,
                max_vals[idx],
                Rational::from_int(1)
                    - (0..mat.rows()).fold(Rational::from_int(0), |acc, row| acc + mat[(row, idx)]),
            )
        })
        .collect();

    let mut sum: Rational = 0.into();
    for r in 0..mat.rows() {
        sum += mat[(r, mat.cols() - 1)];
    }
    let mut best = Rational::from_int(i32::MAX);
    loop {
        if sum > 0.into() && sum < best && sum.to_int().is_some() {
            let mut valid = true;
            // check that all pivot values are integer
            for row in 0..mat.rows() {
                let mut row_val = mat[(row, mat.cols() - 1)];
                for (idx, val, _, _) in state.iter().copied() {
                    if val != 0 {
                        row_val -= mat[(row, idx)] * (val as i32).into();
                    }
                }
                if row_val.to_int().map(|v| v < 0).unwrap_or(true) {
                    valid = false;
                    break;
                }
            }
            if valid {
                best = sum;
            }
        }

        let mut should_continue = false;
        for (_, val, max, step) in &mut state {
            *val += 1;
            sum += *step;
            if val > max {
                sum -= Rational::from_int(*val as i32) * *step;
                *val = 0;
            } else {
                should_continue |= true;
                break;
            }
        }

        if !should_continue {
            break;
        }
    }

    best.to_int().unwrap() as u64
}

fn solve_machine_gaussian(machine: &Machine) -> u64 {
    let mut mat: Mat<Rational> = Mat::new(machine.buttons.len() + 1, machine.joltages.len());
    for (col, button) in machine.buttons.iter().enumerate() {
        for &row in button {
            mat[(row as usize, col)] = 1.into();
        }
    }
    let last_col = machine.buttons.len();
    for (row, &target_joltage) in machine.joltages.iter().enumerate() {
        mat[(row, last_col)] = (target_joltage as i32).into();
    }
    mat.to_rref(true);

    if !is_solvable(&mat) {
        println!("WARNING: Can't solve matrix {:6.2}", mat);
        return 0;
    }

    let free_vars = get_free_vars(&mat);
    let max_vals: Vec<u64> = machine
        .buttons
        .iter()
        .map(|b| {
            b.iter()
                .map(|&wire| machine.joltages[wire as usize] as u64)
                .min()
                .unwrap_or(0)
        })
        .collect();

    solve_machine_gaussian_search(&mat, &free_vars, &max_vals)
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some(machines) = parse(input) {
        let mut num_buttons_pushed = 0;
        for machine in machines {
            let result = solve_machine_gaussian(&machine);
            num_buttons_pushed += result;
        }
        Some(num_buttons_pushed)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
