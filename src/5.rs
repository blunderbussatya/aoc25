use std::io::Read;

fn merge_intervals(mut intervals: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    if intervals.is_empty() {
        return Vec::new();
    }

    intervals.sort_by_key(|interval| interval[0]);

    intervals.into_iter().fold(Vec::new(), |mut acc, current| {
        match acc.last_mut() {
            Some(last) if current[0] <= last[1] => {
                last[1] = std::cmp::max(last[1], current[1]);
            }
            _ => {
                acc.push(current);
            }
        }
        acc
    })
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/5/5.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let (ranges, ids) = contents.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|l| l.split('-').map(|v| v.parse().unwrap()).collect())
        .collect::<Vec<Vec<u64>>>();
    let ids = ids
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let ans = ids
        .iter()
        .map(|id| {
            let in_range = ranges.iter().any(|r| id >= &r[0] && id <= &r[1]);
            if in_range { 1 } else { 0 }
        })
        .sum::<u64>();

    println!("Part1 {}", ans);

    let merged = merge_intervals(ranges);
    let ans = merged.into_iter().fold(0, |acc, r| acc + r[1] - r[0] + 1);
    println!("Part2 {}", ans);
    Ok(())
}
