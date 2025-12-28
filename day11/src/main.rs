use std::{
    collections::{HashMap, HashSet},
    fs,
};

type AdjacencyList = HashMap<u32, Vec<u32>>;

macro_rules! str3_u32 {
    ($s:literal) => {{
        const B: &[u8] = $s.as_bytes();
        ((B[2] as u32) << 16) + ((B[1] as u32) << 8) + (B[0] as u32)
    }};
}

fn parse_input(input: &str) -> Result<AdjacencyList, String> {
    let mut out = HashMap::new();
    input.lines().for_each(|l| {
        let adj = l[5..]
            .split(" ")
            .map(|s| s.as_bytes())
            .map(|b| ((b[2] as u32) << 16) + ((b[1] as u32) << 8) + (b[0] as u32))
            .collect::<Vec<u32>>();
        let b = l[..3].as_bytes();
        out.insert(
            ((b[2] as u32) << 16) + ((b[1] as u32) << 8) + (b[0] as u32),
            adj,
        );
    });
    Ok(out)
}

fn dfs(
    adj: &AdjacencyList,
    memo: &mut HashMap<u32, u64>,
    seen: &mut HashSet<u32>,
    target: u32,
    curr: u32,
) -> u64 {
    if memo.contains_key(&curr) {
        return *memo.get(&curr).unwrap();
    }
    if curr == target {
        return 1;
    }
    let mut total = 0;
    if let Some(v) = adj.get(&curr) {
        total += v
            .iter()
            .map(|k| {
                if seen.contains(k) {
                    return 0;
                }
                seen.insert(*k);
                let t = dfs(adj, memo, seen, target, *k);
                seen.remove(k);
                t
            })
            .sum::<u64>();
    }

    memo.insert(curr, total);
    total
}

fn part1(input: &str) -> Result<String, String> {
    let adj = parse_input(input)?;
    Ok(dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("you")]),
        str3_u32!("out"),
        str3_u32!("you"),
    )
    .to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let adj = parse_input(input)?;

    let fft_to_out = dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("fft"), str3_u32!("dac")]),
        str3_u32!("out"),
        str3_u32!("fft"),
    );
    let dac_to_out = dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("dac"), str3_u32!("fft")]),
        str3_u32!("out"),
        str3_u32!("dac"),
    );
    let dac_to_fft = dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("dac")]),
        str3_u32!("fft"),
        str3_u32!("dac"),
    );
    let fft_to_dac = dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("fft")]),
        str3_u32!("dac"),
        str3_u32!("fft"),
    );
    let svr_to_dac = dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("svr"), str3_u32!("fft")]),
        str3_u32!("dac"),
        str3_u32!("svr"),
    );
    let svr_to_fft = dfs(
        &adj,
        &mut HashMap::new(),
        &mut HashSet::from([str3_u32!("svr"), str3_u32!("dac")]),
        str3_u32!("fft"),
        str3_u32!("svr"),
    );
    Ok((svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out).to_string())
}

fn main() {
    let example1_input =
        fs::read_to_string("./example1.txt").expect("Unable to read from example1.txt");
    let example2_input =
        fs::read_to_string("./example2.txt").expect("Unable to read from example2.txt");
    let part1_input = fs::read_to_string("./p1.txt").expect("Unable to read from p1.txt");

    let _ = part1(&example1_input)
        .map(|s| println!("Answer to part 1 (example input): {}", s))
        .map_err(|err| println!("Error computing part 1 (example input): {}", err));
    let _ = part1(&part1_input)
        .map(|s| println!("Answer to part 1 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 1 (part 1 input): {}", err));
    let _ = part2(&example2_input)
        .map(|s| println!("Answer to part 2 (example input): {}", s))
        .map_err(|err| println!("Error computing part 2 (example input): {}", err));
    let _ = part2(&part1_input)
        .map(|s| println!("Answer to part 2 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 2 (part 1 input): {}", err));
}
