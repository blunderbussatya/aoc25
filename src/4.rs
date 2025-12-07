use std::io::Read;

const DX: [i32; 8] = [1, 0, -1, 0, 1, 1, -1, -1];
const DY: [i32; 8] = [0, 1, 0, -1, 1, -1, -1, 1];

fn main() -> anyhow::Result<()> {
    let filepath = "tests/4/4.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let mut mat = contents
        .lines()
        .map(|v| v.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let n = mat.len();
    let m = mat[0].len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == '.' {
                continue;
            }
            let mut num_present = 0;
            for k in 0..8 {
                let x = i as i32 + DX[k];
                let y = j as i32 + DY[k];
                if x >= 0
                    && x < n as i32
                    && y >= 0
                    && y < m as i32
                    && mat[x as usize][y as usize] == '@'
                {
                    num_present += 1;
                }
            }
            if num_present < 4 {
                ans += 1;
            }
        }
    }
    println!("Part1: {}", ans);

    let mut ans = 0;

    loop {
        let mut changed = false;
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == '.' {
                    continue;
                }
                let mut num_present = 0;
                for k in 0..8 {
                    let x = i as i32 + DX[k];
                    let y = j as i32 + DY[k];
                    if x >= 0
                        && x < n as i32
                        && y >= 0
                        && y < m as i32
                        && mat[x as usize][y as usize] == '@'
                    {
                        num_present += 1;
                    }
                }
                if num_present < 4 {
                    changed = true;
                    mat[i][j] = '.';
                    ans += 1;
                }
            }
        }
        if !changed {
            break;
        }
    }

    println!("Part2: {}", ans);

    Ok(())
}
