use std::fs;

fn largest_joltage_in_line_p1(line: &str) -> Result<u32, String> {
    let mut largest = 0;
    let mut line_iter = line.chars().rev();

    let last_char = line_iter.next().ok_or("Expected a non-empty line")?;
    let mut largest_dig_seen = last_char
        .to_digit(10)
        .ok_or(format!("Expected a digit, found '{}'", last_char))?;

    for ch in line_iter {
        let dig = ch
            .to_digit(10)
            .ok_or(format!("Expected a digit, found '{}'", ch))?;
        largest = largest.max(dig * 10 + largest_dig_seen);
        largest_dig_seen = largest_dig_seen.max(dig);
    }
    Ok(largest)
}

/// In order to compute the largest 12 dig, we need to know the largest
/// 12/11 dig on str[1..], which requires knowledge of str[2..] and so on
/// until str[n-12..]. In order to solve the problem on str[n-12], we need
/// Recursively, this looks like
/// return max(line[0] + largest_n(line[1..], n-1), largest_n(line[1..], n))
fn largest_joltage_in_line_p2(line: &str) -> Result<u64, String> {
    // First we compute the largest 11 digit number in line[(n-12)..]
    let digs = line
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .ok_or(format!("Expected a digit, found '{}'", ch))
                .map(|d| d as u64)
        })
        .collect::<Result<Vec<u64>, String>>()?;

    let n = digs.len();
    if n < 12 {
        return Err(format!("Line must be at least 12 characters long"));
    }

    let mut largests = vec![0; 12];
    for i in (0..n).rev() {
        for j in (0..largests.len().min(n - i)).rev() {
            if j == 0 {
                largests[0] = digs[i].max(largests[0]);
                continue;
            }
            largests[j] = largests[j].max(digs[i] * 10_u64.pow(j as u32) + largests[j - 1]);
        }
    }
    Ok(largests[largests.len() - 1])
}

fn part1(input: &str) -> Result<String, String> {
    let mut total = 0_u64;
    for line in input.lines() {
        total += largest_joltage_in_line_p1(line)
            .map_err(|e| format!("Error computing largest joltage in line '{}': {}", line, e))?
            as u64;
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut total = 0_u64;
    for line in input.lines() {
        total += largest_joltage_in_line_p2(line)
            .map_err(|e| format!("Error computing largest joltage in line '{}': {}", line, e))?;
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
