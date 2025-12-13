use std::fs;

fn part1(input: &str) -> Result<String, String> {
    let chart = input.lines().collect::<Vec<&str>>();
    let mut total = 0;
    for (i, line) in chart.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '@' {
                continue;
            }
            let mut neighbours = 0;
            'outer: for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    chart
                        .get((i as isize + di) as usize)
                        .and_then(|l| l.chars().nth((j as isize + dj) as usize))
                        .map(|c| {
                            if c == '@' {
                                neighbours += 1;
                            }
                        });
                    if neighbours >= 4 {
                        break 'outer;
                    }
                }
            }
            if neighbours < 4 {
                total += 1;
            }
        }
    }

    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut chart = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut total = 0;

    loop {
        let mut next_chart = chart.clone();
        let mut subtotal = 0;
        for (i, line) in chart.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c != '@' {
                    continue;
                }
                let mut neighbours = 0;
                'outer: for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        chart
                            .get((i as isize + di) as usize)
                            .and_then(|l| l.get((j as isize + dj) as usize))
                            .map(|c| {
                                if *c == '@' {
                                    neighbours += 1;
                                }
                            });
                        if neighbours >= 4 {
                            break 'outer;
                        }
                    }
                }
                if neighbours < 4 {
                    subtotal += 1;
                    next_chart[i][j] = '.';
                }
            }
        }
        if subtotal == 0 {
            break;
        }
        total += subtotal;
        chart = next_chart;
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
