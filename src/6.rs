use std::io::Read;

fn transpose<T: Clone>(mat: &[Vec<T>]) -> Vec<Vec<T>> {
    if mat.is_empty() {
        return Vec::new();
    }

    let m = mat[0].len();

    (0..m)
        .map(|j| mat.iter().map(|row| row[j].clone()).collect())
        .collect()
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/6/6.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let lines = contents.lines().collect::<Vec<&str>>();

    let input = lines[0..lines.len() - 1]
        .iter()
        .map(|l| {
            l.trim()
                .split(' ')
                .filter_map(|l| {
                    let t = l.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(t.parse().unwrap())
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();
    let ops = lines
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|l| {
            let t = l.trim();
            if t.is_empty() { None } else { Some(t) }
        })
        .collect::<Vec<_>>();

    let input_t = transpose(&input);

    let ans = ops
        .iter()
        .enumerate()
        .map(|(idx, op)| match *op {
            "+" => input_t[idx].iter().sum::<u64>(),
            "*" => input_t[idx].iter().product::<u64>(),
            _ => panic!("Unknown operation"),
        })
        .sum::<u64>();
    println!("Part1 {}", ans);

    let input = lines[..lines.len() - 1]
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let input_t = transpose(&input);

    let input_t = input_t
        .split(|v| v.iter().all(|&c| c == ' '))
        .collect::<Vec<_>>();

    let ans = ops
        .iter()
        .enumerate()
        .map(|(idx, op)| match *op {
            "+" => input_t[idx]
                .iter()
                .map(|v| {
                    v.iter()
                        .collect::<String>()
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(0)
                })
                .sum::<u64>(),
            "*" => input_t[idx]
                .iter()
                .map(|v| {
                    v.iter()
                        .collect::<String>()
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(0)
                })
                .product::<u64>(),
            _ => panic!("Unknown operation"),
        })
        .sum::<u64>();

    println!("Part2 {}", ans);

    Ok(())
}
