use std::io::Read;

// Gets the first maximum value in a slice along with its index
fn max_with_idx<T: Ord + std::fmt::Debug>(v: &[T]) -> (usize, &T) {
    let (idx, val) = v
        .iter()
        .rev()
        .enumerate()
        .max_by_key(|(_, val)| *val)
        .map(|(idx, val)| (idx, val))
        .unwrap();
    (v.len() - idx, val)
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/3/3.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let input = contents
        .lines()
        .map(|c| c.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let ans = input
        .iter()
        .map(|v| {
            let (idx, mx) = max_with_idx(&v[..v.len() - 1]);
            let nmx = v[idx..].iter().max().unwrap();
            let base = '0' as u64;
            let f1 = (*mx as u64) - base;
            let f2 = (*nmx as u64) - base;
            f1 * 10 + f2
        })
        .sum::<u64>();
    println!("Part1: {}", ans);
    
    let mut ans2 = 0;
    for v in input {
        let mut cur = 0;
        let mut idx = 0;
        for i in 0..12 {
            let (nidx, val) = max_with_idx(&v[idx..(v.len() + i - 11)]);
            let base = '0' as u64;
            let f = (*val as u64) - base;
            cur = cur * 10 + f;
            idx += nidx;
        }
        ans2 += cur;
    }
    println!("Part2: {}", ans2);
    Ok(())
}
