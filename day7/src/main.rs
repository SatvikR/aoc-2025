use std::{fs, iter, mem};

fn part1(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let top = lines
        .next()
        .ok_or_else(|| format!("Expected non-empty input."))?;
    lines
        .next()
        .ok_or_else(|| format!("Expected at least 2 lines of input."))?;

    let mut total = 0_u32;
    let mut state = vec![false; top.len()];
    for (i, c) in top.chars().enumerate() {
        if c == 'S' {
            state[i] = true;
            break;
        }
    }

    while let Some(line) = lines.next() {
        let mut next_state = state.clone();
        lines
            .next()
            .ok_or_else(|| format!("Expected a line of dots every other line, got EOF.",))?;

        for ((i, state_beam), splitter) in iter::zip(state.iter().enumerate(), line.chars()) {
            if !state_beam || splitter != '^' {
                continue;
            }
            total += 1;
            next_state[i] = false;
            if i > 0 {
                next_state[i - 1] = true;
            }
            if i < next_state.len() - 1 {
                next_state[i + 1] = true;
            }
        }
        mem::swap(&mut state, &mut next_state);
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let top = lines
        .next()
        .ok_or_else(|| format!("Expected non-empty input."))?;
    lines
        .next()
        .ok_or_else(|| format!("Expected at least 2 lines of input."))?;

    let mut state = vec![(false, 0_u64); top.len()];
    for (i, c) in top.chars().enumerate() {
        if c == 'S' {
            state[i] = (true, 1);
            break;
        }
    }

    while let Some(line) = lines.next() {
        let mut next_state = state.clone();
        lines
            .next()
            .ok_or_else(|| format!("Expected a line of dots every other line, got EOF.",))?;

        for ((i, state_beam), splitter) in iter::zip(state.iter().enumerate(), line.chars()) {
            if !state_beam.0 || splitter != '^' {
                continue;
            }
            next_state[i].0 = false;
            next_state[i].1 = 0;
            if i > 0 {
                next_state[i - 1].0 = true;
                next_state[i - 1].1 += state_beam.1;
            }
            if i < next_state.len() - 1 {
                next_state[i + 1].0 = true;
                next_state[i + 1].1 += state_beam.1;
            }
        }
        mem::swap(&mut state, &mut next_state);
    }
    Ok(state
        .iter()
        .map(|(_, preds)| preds)
        .sum::<u64>()
        .to_string())
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
