use std::{collections::HashSet, io::Read};

fn main() -> anyhow::Result<()> {
    let filepath = "tests/2/2.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let input = contents
        .split(',')
        .map(|s| {
            s.split('-')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = (1..100_001)
        .map(|i| {
            let cur = format!("{i}{i}").parse::<u64>().unwrap();
            let in_range = input.iter().any(|v| v[0] <= cur && v[1] >= cur);
            if in_range { cur } else { 0 }
        })
        .sum::<u64>();

    println!("Part1: {ans}");
    let mut unique = HashSet::new();

    for i in 1..100_001 {
        let mut cur = format!("{i}");
        for _ in 1..10 {
            cur = format!("{cur}{i}");
            if cur.len() >= 12 {
                break;
            }
            let cur_num = cur.parse::<u64>().unwrap();
            let in_range = input.iter().any(|v| v[0] <= cur_num && v[1] >= cur_num);
            if in_range {
                unique.insert(cur_num);
            }
        }
    }
    println!("Part2: {}", unique.into_iter().sum::<u64>());
    Ok(())
}
