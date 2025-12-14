use std::{fs, str::Chars};

enum Operation {
    Add,
    Multiply,
}

type Computation = Vec<(Vec<u64>, Operation)>;

fn parse_input_part_1(input: &str) -> Result<Computation, String> {
    let lines = input.lines().collect::<Vec<&str>>();
    let ops = lines[lines.len() - 1]
        .split(' ')
        .filter(|c| *c != "")
        .map(|o| match o.trim() {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            _ => Err(format!("Expected '+' or '*', got {}", o)),
        })
        .collect::<Result<Vec<Operation>, String>>()?;

    let mut computation = Vec::with_capacity(ops.len());
    for o in ops {
        computation.push((Vec::with_capacity(lines.len() - 1), o));
    }

    for line in &lines[..lines.len() - 1] {
        let nums = line
            .split(' ')
            .filter(|c| *c != "")
            .map(|c| {
                c.parse::<u64>()
                    .map_err(|e| format!("Unable to parse u64 '{}': {}", c, e))
            })
            .collect::<Result<Vec<u64>, String>>()?;
        for (i, num) in nums.iter().enumerate() {
            computation[i].0.push(*num);
        }
    }

    Ok(computation)
}

fn parse_input_part_2(input: &str) -> Result<Computation, String> {
    let mut lines = input.lines().map(|s| s.chars()).collect::<Vec<Chars>>();
    if lines.len() == 0 {
        return Err(format!("Expected a non-empty input."));
    }

    let mut computation = Vec::new();
    let mut ops_iter = lines.pop().and_then(|i| Some(i.peekable())).unwrap();
    loop {
        // Read in the operator.
        let op = match ops_iter.next() {
            Some('*') => Ok(Operation::Multiply),
            Some('+') => Ok(Operation::Add),
            Some(_x) => Err(format!("Expected a '*' or a '+', got '{}'", _x)),
            None => break,
        }?;
        // Read in spaces until we can peek a non-space char or EOF to compute number of digs.
        let mut digs = 0;
        loop {
            match ops_iter.peek() {
                Some(' ') => {
                    digs += 1;
                    ops_iter.next();
                }
                Some(_) => break,
                None => {
                    digs += 1;
                    break;
                }
            }
        }
        // We know we will need digs numbers for this column.
        let mut column = vec![0_u64; digs];
        // Consume digs+1 chars from each line iter.
        for line_iter in &mut lines {
            for place in 0..=digs {
                match line_iter.next() {
                    d @ Some('0'..='9') => {
                        let dig = d.unwrap().to_digit(10).unwrap() as u64;
                        if column[place] == 0 {
                            column[place] = dig;
                        } else {
                            column[place] *= 10;
                            column[place] += dig;
                        }
                    }
                    Some(' ') | None => (),
                    Some(_x) => {
                        return Err(format!("Expected digit, space, or EOF, got: '{}'", _x));
                    }
                }
            }
        }
        computation.push((column, op));
    }

    Ok(computation)
}

fn part1(input: &str) -> Result<String, String> {
    let computation =
        parse_input_part_1(input).map_err(|e| format!("Unable to parse input: {}", e))?;
    let mut total = 0_u64;
    for (nums, op) in computation {
        let mut subtotal = *nums
            .get(0)
            .ok_or_else(|| format!("Expected a non-empty list of numbers in each column."))?;
        for num in nums.iter().skip(1) {
            match op {
                Operation::Add => subtotal += num,
                Operation::Multiply => subtotal *= num,
            }
        }
        total += subtotal;
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let computation =
        parse_input_part_2(input).map_err(|e| format!("Unable to parse input: {}", e))?;
    let mut total = 0_u64;
    for (nums, op) in computation {
        let mut subtotal = *nums
            .get(0)
            .ok_or_else(|| format!("Expected a non-empty list of numbers in each column."))?;
        for num in nums.iter().skip(1) {
            match op {
                Operation::Add => subtotal += num,
                Operation::Multiply => subtotal *= num,
            }
        }
        total += subtotal;
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
