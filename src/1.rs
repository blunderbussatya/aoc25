use core::panic;
use std::io::Read;

const MOD: i32 = 100;

fn main() -> anyhow::Result<()> {
    let filepath = "tests/1/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let ans = contents.lines().fold((50, 0), |(acc, zeros), line| {
        let op = &line[0..1];
        let num = line[1..].parse::<i32>().unwrap();
        let acc_n = match op {
            "R" => (acc + num) % MOD,
            "L" => (acc - num + MOD) % MOD,
            _ => panic!("unexpected operation"),
        };
        (acc_n, zeros + if acc_n == 0 { 1 } else { 0 })
    });

    println!("Part1: {}", ans.1);

    let ans = contents.lines().fold((50, 0), |(acc, zeros), line| {
        let op = &line[0..1];
        let num = line[1..].parse::<i32>().unwrap();
        let c = num / MOD;
        let m = num % MOD;
        let acc_n = match op {
            "R" => acc + m,
            "L" => acc - m,
            _ => panic!("unexpected operation"),
        };
        (
            (acc_n + MOD) % MOD,
            zeros
                + c
                + if acc != 0 && (acc_n >= MOD || acc_n <= 0) {
                    1
                } else {
                    0
                },
        )
    });

    println!("Part2: {}", ans.1);

    Ok(())
}
