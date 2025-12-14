use std::{collections, fs};

struct Pantry {
    ranges: Vec<(u64, u64)>,
    items: Vec<u64>,
}

fn parse_input(input: &str) -> Result<Pantry, String> {
    let mut sections = input.split("\n\n");
    let ranges_section = sections
        .next()
        .ok_or_else(|| "Missing ranges section".to_string())?;
    let items_section = sections
        .next()
        .ok_or_else(|| "Missing items section".to_string())?;

    let ranges = ranges_section
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range line: {}", line));
            }
            let start = parts[0]
                .parse::<u64>()
                .map_err(|_| format!("Invalid start of range: {}", parts[0]))?;
            let end = parts[1]
                .parse::<u64>()
                .map_err(|_| format!("Invalid end of range: {}", parts[1]))?;
            Ok((start, end))
        })
        .collect::<Result<Vec<(u64, u64)>, String>>()?;

    let items = items_section
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|_| format!("Invalid item: {}", line))
        })
        .collect::<Result<Vec<u64>, String>>()?;

    Ok(Pantry { ranges, items })
}

fn part1(input: &str) -> Result<String, String> {
    let pantry = parse_input(input).map_err(|e| format!("Unable to parse input: {}", e))?;
    let mut total = 0;
    for item in pantry.items {
        for (start, end) in &pantry.ranges {
            if item >= *start && item <= *end {
                total += 1;
                break;
            }
        }
    }

    Ok(total.to_string())
}

/// i'm pretty sure this is a leetcode question.
/// for each range we want to add [a, b]:
/// - find the smallest range [a', b'] that has a' <= a.
/// Either create a range or set the curr range as the target to extend. If b <= b', we're done.
/// if not, check if b < a''. if so, extend this range. If not, merge this range and the next range and try again.
fn part2(input: &str) -> Result<String, String> {
    let pantry = parse_input(input).map_err(|e| format!("Unable to parse input: {}", e))?;
    let mut total = 0_u64;
    // LinkedList would be ideal here, but the cursor functionality is still
    // in nightly releases only.
    let mut ranges = collections::VecDeque::new();
    ranges.push_back(pantry.ranges[0]);

    for (a, b) in pantry.ranges.iter().skip(1) {
        let mut curr_range = None;
        for i in 0..ranges.len() {
            let (start, end) = ranges[i];
            if *a < start {
                ranges.insert(i, (*a, *b));
                curr_range = Some(i);
                break;
            }
            if *a >= start && *a <= end {
                if *b > end {
                    ranges[i].1 = *b;
                }
                curr_range = Some(i);
                break;
            }
        }
        if curr_range.is_none() {
            ranges.push_back((*a, *b));
            continue;
        }
        let curr_range = curr_range.unwrap();
        let i = curr_range + 1;
        while i < ranges.len() {
            // b is either:
            // - behind ranges[i] (just exit)
            // - in ranges[i] (update curr_range end to ranges[i] end and remove ranges[i])
            // - ahead of ranges[i] (merge and continue)
            if *b < ranges[i].0 {
                break;
            }
            if ranges[i].0 <= *b && *b <= ranges[i].1 {
                ranges[curr_range].1 = ranges[i].1;
                ranges.remove(i);
                break;
            }
            ranges.remove(i);
        }
    }
    for (start, end) in ranges {
        total += end - start + 1;
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
