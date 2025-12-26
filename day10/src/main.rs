use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
use std::fs;

struct PartOneDiagram {
    lights: u32,
    buttons: Vec<u32>,
}

#[derive(Debug)]
struct PartTwoDiagram {
    rows: Vec<Vec<f64>>,
    joltages: Vec<f64>,
}

fn parse_input_p1(input: &str) -> Result<Vec<PartOneDiagram>, String> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<&str>>();
            if parts.len() < 2 {
                return Err(format!("Expected at least 2 parts per line."));
            }

            let mut lights = 0;
            parts[0][1..parts[0].len() - 1]
                .chars()
                .enumerate()
                .for_each(|(i, c)| {
                    if c == '#' {
                        lights |= 1 << i;
                    }
                });

            let mut buttons = vec![0; parts.len() - 2];
            for (i, s) in parts.iter().skip(1).take(buttons.len()).enumerate() {
                for c in s[1..s.len() - 1].split(',') {
                    buttons[i] |= 1
                        << (c
                            .parse::<u32>()
                            .map_err(|e| format!("Expected a digit, got '{}': {}", c, e))?);
                }
            }

            Ok(PartOneDiagram { lights, buttons })
        })
        .collect::<Result<Vec<PartOneDiagram>, String>>()
}

fn parse_input_p2(input: &str) -> Result<Vec<PartTwoDiagram>, String> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<&str>>();
            if parts.len() < 2 {
                return Err(format!("Expected at least 2 parts per line."));
            }

            let joltages = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
                .split(",")
                .map(|c| {
                    c.parse::<f64>()
                        .map_err(|e| format!("Expected a number, got '{}': {}", c, e))
                })
                .collect::<Result<Vec<f64>, String>>()?;

            let num_buttons = parts.len() - 2;
            let mut rows = vec![vec![0_f64; num_buttons]; joltages.len()];
            for (i, s) in parts.iter().skip(1).take(parts.len() - 2).enumerate() {
                for c in s[1..s.len() - 1].split(',') {
                    // set the i'th bit of the c'th "button".
                    let row = c
                        .parse::<usize>()
                        .map_err(|e| format!("Expected a digit, got '{}': {}", c, e))?;
                    rows[row][i] = 1_f64;
                }
            }

            Ok(PartTwoDiagram { rows, joltages })
        })
        .collect::<Result<Vec<PartTwoDiagram>, String>>()
}

/// Recursive function to solve problems.
fn backtrack(target: u32, nums: &Vec<u32>, curr: usize) -> Option<u32> {
    if target == 0 {
        return Some(0);
    }
    if curr >= nums.len() {
        return None;
    }

    match (
        backtrack(target ^ nums[curr], nums, curr + 1),
        backtrack(target, nums, curr + 1),
    ) {
        (Some(a), Some(b)) => Some((a + 1).min(b)),
        (Some(a), None) => Some(a + 1),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn part1(input: &str) -> Result<String, String> {
    let problems = parse_input_p1(input).map_err(|e| format!("Error parsing input: {}", e))?;

    let mut total: u32 = 0;
    for problem in problems {
        total += backtrack(problem.lights, &problem.buttons, 0)
            .ok_or_else(|| format!("Unable to find solution to one of the problems."))?;
    }

    Ok(total.to_string())
}

fn gaussian_elimination(diagram: &PartTwoDiagram) -> Result<f64, String> {
    variables! {
        problem:
    };
    let x = problem.add_vector(variable().integer().min(0), diagram.rows[0].len());
    let objective: Expression = x.iter().sum();
    let mut model = problem.minimise(objective).using(default_solver);

    for i in 0..diagram.rows.len() {
        let mut expr: Expression = 0.into();
        for (j, x_j) in x.iter().enumerate() {
            expr += diagram.rows[i][j] * (*x_j);
        }
        model = model.with(expr.eq(diagram.joltages[i]));
    }
    let solution = model
        .solve()
        .map_err(|e| format!("Unable to solve diagram: {}", e))?;
    Ok(x.iter().map(|x_i| solution.value(*x_i)).sum())
}

/// (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
/// a   b      c    d   e      f
/// Becomes:
/// 3 = 0a + 0b + 0c + 0d + 1e + 1f
/// 5 = 0a + 1b + 0c + 0d + 0e + 1f
/// 4 = 0a + 0b + 1c + 1d + 1e + 0f
/// 7 = 1a + 1b + 0c + 1d + 0e + 0f
///
/// 0 0 0 0 1 1 | 3
/// 0 1 0 0 0 1 | 5
/// 0 0 1 1 1 0 | 4
/// 1 1 0 1 0 0 | 7
///
/// then ret the soln that
/// minimizes a + b + c + d + e + f
/// under the constraint that they are all non-neg.
fn part2(input: &str) -> Result<String, String> {
    let problems = parse_input_p2(input).map_err(|e| format!("Error parsing input: {}", e))?;

    let mut total: u32 = 0;
    for mut problem in problems {
        total += gaussian_elimination(&mut problem)?.floor() as u32;
    }
    Ok(total.to_string())
}

fn main() {
    let example_input =
        fs::read_to_string("./example.txt").expect("Unable to read from example.txt");
    let part1_input = fs::read_to_string("./p1.txt").expect("Unable to read from p1.txt");

    let _ = part1(&example_input)
        .map(|s| println!("Answer to part 1 (example input): {}", s))
        .map_err(|err| println!("Error computing part 1 (example input): {}", err));
    let _ = part1(&part1_input)
        .map(|s| println!("Answer to part 1 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 1 (part 1 input): {}", err));
    let _ = part2(&example_input)
        .map(|s| println!("Answer to part 2 (example input): {}", s))
        .map_err(|err| println!("Error computing part 2 (example input): {}", err));
    let _ = part2(&part1_input)
        .map(|s| println!("Answer to part 2 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 2 (part 1 input): {}", err));
}
