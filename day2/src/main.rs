use std::{cmp, collections::HashSet, fs};

/// First figure out the number of digits we can work with,
/// and ignore odd lengths. From there, we want the smallest
/// repeated number that fits in the range. Say we are working
/// with 2n digits. Then in the range[a, b], we want the smallest
/// x such that:
/// x * 10^n + x >= a
/// x(10^n + 1) >= a
/// x >= a / (10^n + 1)
fn invalid_ids_in_range_p1(a: u64, b: u64) -> u64 {
    let lower_bound_digs = {
        let l = a.ilog10() + 1;
        if l % 2 == 0 { l } else { l + 1 }
    };

    let mut total = 0_u64;
    let mut n = lower_bound_digs / 2;
    let effective_lower_bound = cmp::max(10_u64.pow(2 * n - 1), a);
    let mut x = ((effective_lower_bound as f64) / (10_f64.powf(n.into()) + 1.0)).ceil() as u64;
    loop {
        let c = x * 10_u64.pow(n) + x;
        if c > b {
            break;
        }
        total += c;
        x += 1;
        n = x.ilog(10) + 1;
    }
    total
}

/// We only want to look at factors of the number of digits
/// in the range. This means we now care about odd lengths
/// as well. If a number evenly divides the number of digits,
/// we can find the minimum repetetive number of that length:
/// take the repeated number x, and apply x := x * 10^n + x
/// repeatedly until we reach the desired length. This wouuld
/// be the number of desired digits / digits of x.
fn invalid_ids_in_range_p2(a: u64, b: u64) -> u64 {
    let lower_bound_digs = a.ilog10() + 1;
    let upper_bound_digs = b.ilog10() + 1;

    let mut total = 0_u64;
    let construct_repeated_number = |x: u64, digs: u32, digs_x: u32| -> u64 {
        let mut result = x;
        for _ in 0..(digs / digs_x - 1) {
            result = result * 10_u64.pow(digs_x) + x;
        }
        result
    };

    // there's prob a way to get around ussing this.
    let mut seen = HashSet::new();

    for digs in lower_bound_digs..=upper_bound_digs {
        for i in 1..=digs / 2 {
            if digs % i != 0 {
                continue;
            }

            // not sure if there's a quick way to get the start
            // of the range, so we can just guess the first i digits
            // of the lower bound.
            let effective_lower_bound = cmp::max(10_u64.pow(digs - 1), a);
            let mut x: u64 = effective_lower_bound / 10_u64.pow(digs - i);

            while x.ilog10() + 1 == i {
                let c = construct_repeated_number(x, digs, i);
                if c > b {
                    break;
                }
                if c >= a && !seen.contains(&c) {
                    total += c;
                    seen.insert(c);
                }
                x += 1;
            }
        }
    }
    total
}

fn part1(input: &str) -> Result<String, String> {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut total = 0;
    for range in ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        if bounds.len() != 2 {
            return Err(format!("Invalid range: {}", range));
        }

        let a = bounds[0]
            .parse::<u64>()
            .map_err(|e| format!("Unable to parse lower bound '{}': {}", bounds[0], e))?;
        let b = bounds[1]
            .parse::<u64>()
            .map_err(|e| format!("Unable to parse upper bound '{}': {}", bounds[1], e))?;
        total += invalid_ids_in_range_p1(a, b);
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut total = 0;
    for range in ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        if bounds.len() != 2 {
            return Err(format!("Invalid range: {}", range));
        }

        let a = bounds[0]
            .parse::<u64>()
            .map_err(|e| format!("Unable to parse lower bound '{}': {}", bounds[0], e))?;
        let b = bounds[1]
            .parse::<u64>()
            .map_err(|e| format!("Unable to parse upper bound '{}': {}", bounds[1], e))?;
        total += invalid_ids_in_range_p2(a, b);
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
