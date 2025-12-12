use std::fs;

fn part1(input: &str) -> Result<String, String> {
    let moves: Vec<&str> = input.split("\n").collect();
    let mut pos = 50;
    let mut total = 0;

    for m in moves {
        let delta = (&m[1..])
            .parse::<i32>()
            .map_err(|err| format!("Unable to parse lock delta '{}': {}", m, err))?;

        pos = match m.chars().nth(0) {
            Some('L') => (pos - delta).rem_euclid(100),
            Some('R') => (pos + delta).rem_euclid(100),
            _ => return Err(format!("Unable to parse move: '{}'", m)),
        };
        if pos == 0 {
            total += 1;
        }
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let moves: Vec<&str> = input.split("\n").collect();
    let mut pos = 50;
    let mut total = 0;

    for m in moves {
        let raw_delta = (&m[1..])
            .parse::<i32>()
            .map_err(|err| format!("Unable to parse lock delta '{}': {}", m, err))?;

        let delta = match m.chars().nth(0) {
            Some('L') => -raw_delta,
            Some('R') => raw_delta,
            _ => return Err(format!("Unable to parse move: '{}'", m)),
        };
        let dist_to_0 = match delta.signum() {
            1 => (100 - pos) % 100,
            -1 => pos % 100,
            _ => 0,
        };
        total += (raw_delta - dist_to_0) / 100;
        if raw_delta >= dist_to_0 && dist_to_0 != 0 {
            total += 1;
        }
        pos = (pos + delta).rem_euclid(100);
    }
    Ok(total.to_string())
}

fn main() {
    let example_input =
        fs::read_to_string("./example.txt").expect("Unable to read from example.txt");
    let part1_input = fs::read_to_string("./p1.txt").expect("Unable to read from input.txt");

    let _ = part1(&example_input)
        .map(|s| println!("Answer to part 1 (example input): {}", s))
        .map_err(|err| println!("Error computing part 1 (example input): {}", err));
    let _ = part1(&part1_input)
        .map(|s| println!("Answer to part 1 (p1 input): {}", s))
        .map_err(|err| println!("Error computing part 1 (p1 input): {}", err));
    let _ = part2(&example_input)
        .map(|s| println!("Answer to part 2 (example input): {}", s))
        .map_err(|err| println!("Error computing part 2 (example input): {}", err));
    let _ = part2(&part1_input)
        .map(|s| println!("Answer to part 2 (p1 input): {}", s))
        .map_err(|err| println!("Error computing part 2 (p1 input): {}", err));
}
