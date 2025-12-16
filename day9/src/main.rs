use std::{collections::BTreeMap, fs};

fn parse_input(input: &str) -> Result<Vec<(u64, u64)>, String> {
    input
        .lines()
        .map(|l| {
            let coords = l
                .split(',')
                .map(|c| {
                    c.parse::<u64>()
                        .map_err(|e| format!("Expected u64, got '{}': {}", c, e))
                })
                .collect::<Result<Vec<u64>, String>>()?;
            if coords.len() != 2 {
                return Err(format!(
                    "Error parsing line: '{}': expected two coordinates on each line.",
                    l
                ));
            }
            Ok((coords[0], coords[1]))
        })
        .collect::<Result<Vec<(u64, u64)>, String>>()
}

fn part1(input: &str) -> Result<String, String> {
    let coords = parse_input(input).map_err(|e| format!("Error parsing input: {}", e))?;
    let mut max_area_seen = 0_u64;
    for i in 0..coords.len() {
        for j in i..coords.len() {
            max_area_seen = max_area_seen.max(
                (1 + coords[i].0.abs_diff(coords[j].0)) * (1 + coords[i].1.abs_diff(coords[j].1)),
            )
        }
    }

    Ok(max_area_seen.to_string())
}

fn pip(x0: u64, y0: u64, vertical_edges: &BTreeMap<u64, Vec<(u64, u64)>>) -> bool {
    let mut corner_ok = false;
    for (x, edges) in vertical_edges.range(x0..) {
        // Find the edge that this corner actually touches.
        for edge in edges {
            if edge.0 <= y0 && y0 <= edge.1 {
                if *x == x0 {
                    // (x0, y0) is on the boundary
                    return true;
                }
                corner_ok = !corner_ok;
                break;
            }
        }
    }
    return corner_ok;
}

/// What if we could know all the polygon edges that intersect a given candidate rectangle?
/// Every edge is either perfectly vertical or perfectly horizontal.
/// Consider:
/// #XXX#...#XXXX#
/// X...X...X....X
/// X...#XXX#....X
/// X............X
/// #XXXXXXXXXXXX#
/// We can see the top left and bottom right do not form a valid rectangle in this case. BUT:
/// #XXX##XXXX#
/// X...XX....X
/// X...##....X
/// X.........X
/// #XXXXXXXXX#
/// This is perfectly fine. In this case, we have two vertical edges going through the rectangle,
/// and two horizontal edges. They have 1 space between them each, which means they're okay, whereas
/// above they have more than one, so that's not okay. Does that work?
fn part2(input: &str) -> Result<String, String> {
    let coords = parse_input(input).map_err(|e| format!("Error parsing input: {}", e))?;
    // We want to be able to get a list of line segments given an x-range or y-range.
    let mut vertical_edges = BTreeMap::<u64, Vec<(u64, u64)>>::new();
    let mut horizontal_edges = BTreeMap::<u64, Vec<(u64, u64)>>::new();

    for i in 0..coords.len() {
        let j = if i == 0 { coords.len() - 1 } else { i - 1 };
        if coords[i].0 == coords[j].0 {
            let e = (coords[i].1.min(coords[j].1), coords[i].1.max(coords[j].1));
            vertical_edges
                .entry(coords[i].0)
                .and_modify(|v| v.push(e))
                .or_insert(vec![e]);
        } else {
            let e = (coords[i].0.min(coords[j].0), coords[i].0.max(coords[j].0));
            horizontal_edges
                .entry(coords[i].1)
                .and_modify(|v| v.push(e))
                .or_insert(vec![e]);
        }
    }

    let mut max_area_seen = 0;
    for i in 0..coords.len() {
        'outer: for j in i + 1..coords.len() {
            let x0 = coords[i].0.min(coords[j].0);
            let x1 = coords[i].0.max(coords[j].0);
            let y0 = coords[i].1.min(coords[j].1);
            let y1 = coords[i].1.max(coords[j].1);
            // we know lines are not the right answer. so for simplicity,
            if x0 == x1 || y0 == y1 {
                continue;
            }
            // Check if corners are in polygon. Actually check the inner corners.
            if !pip(x0 + 1, y0 + 1, &vertical_edges)
                || !pip(x1 - 1, y1 - 1, &vertical_edges)
                || !pip(x0 + 1, y1 - 1, &vertical_edges)
                || !pip(x1 - 1, y0 + 1, &vertical_edges)
            {
                continue;
            }
            // Now check for the intersections into the rectangle. We know
            // all of the corners are valid.
            let mut last_intersection_coord = None;
            for (x, edges) in vertical_edges.range(x0 + 1..x1) {
                for edge in edges {
                    if edge.0 >= y1 || edge.1 <= y0 {
                        continue;
                    }
                    match last_intersection_coord {
                        Some(nx) => {
                            if x - nx > 1 {
                                continue 'outer;
                            }
                            last_intersection_coord = None;
                        }
                        None => last_intersection_coord = Some(x),
                    }
                }
            }

            let mut last_intersection_coord = None;
            for (y, edges) in horizontal_edges.range(y0 + 1..y1) {
                for edge in edges {
                    if edge.0 >= x1 || edge.1 <= x0 {
                        continue;
                    }
                    match last_intersection_coord {
                        Some(ny) => {
                            if y - ny > 1 {
                                continue 'outer;
                            }
                            last_intersection_coord = None;
                        }
                        None => last_intersection_coord = Some(y),
                    }
                }
            }

            max_area_seen = max_area_seen.max(
                (1 + coords[i].0.abs_diff(coords[j].0)) * (1 + coords[i].1.abs_diff(coords[j].1)),
            );
        }
    }

    Ok(max_area_seen.to_string())
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
