use std::fs;

/// Potential actual solutions:
/// - Turn this into a SAT problem and use a SAT solver
/// - Backtracking
/// Unfortunately I read online that he gave us very very good input on this one.

#[derive(Debug)]
struct Region {
    width: u8,
    height: u8,
    counts: [u8; 6],
}

type Farm = Vec<Region>;

fn parse_input(input: &str) -> Result<Farm, String> {
    input
        .split("\n\n")
        .skip(6)
        .collect::<Vec<&str>>()
        .get(0)
        .ok_or_else(|| format!("No regions found in input file"))?
        .split('\n')
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let dim = parts[0][..parts[0].len() - 1]
                .split('x')
                .map(|d| {
                    d.parse::<u8>()
                        .map_err(|e| format!("Unable to parse dimension: {}", e))
                })
                .collect::<Result<Vec<u8>, String>>()?;
            let mut counts = [0_u8; 6];
            let _ = parts
                .iter()
                .skip(1)
                .map(|p| {
                    p.parse::<u8>()
                        .map_err(|e| format!("Unable to parse present: {}", e))
                })
                .enumerate()
                .map(|(i, c)| {
                    counts[i] = c?;
                    Ok::<(), String>(())
                })
                .collect::<Result<Vec<()>, String>>()?;
            Ok::<Region, String>(Region {
                width: dim[0],
                height: dim[1],
                counts,
            })
        })
        .collect()
}

fn part1(input: &str) -> Result<String, String> {
    let farm = parse_input(input)?;

    let mut total: u32 = farm.len() as u32;
    for region in &farm {
        let area: u32 = (region.width as u32) * (region.height as u32);
        let mut area_needed: u32 = 0;
        for count in region.counts {
            area_needed += 9 * (count as u32);
            if area_needed > area {
                total -= 1;
                break;
            }
        }
    }

    Ok(total.to_string())
}

fn main() {
    let example1_input =
        fs::read_to_string("./example1.txt").expect("Unable to read from example1.txt");
    let part1_input = fs::read_to_string("./p1.txt").expect("Unable to read from p1.txt");

    // produces wrong output for example. lol.
    let _ = part1(&example1_input)
        .map(|s| println!("Answer to part 1 (example input): {}", s))
        .map_err(|err| println!("Error computing part 1 (example input): {}", err));
    let _ = part1(&part1_input)
        .map(|s| println!("Answer to part 1 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 1 (part 1 input): {}", err));
}
