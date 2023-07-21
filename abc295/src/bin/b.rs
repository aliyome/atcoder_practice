use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r]
    };

    let mut ans: Vec<Vec<char>> = b.clone();

    // 全探索
    let mut bomb = |y: usize, x: usize, k: usize| {
        for i in 0..r {
            for j in 0..c {
                if (i as i32 - y as i32).abs() + (j as i32 - x as i32).abs() <= k as i32 {
                    ans[i][j] = '.';
                }
            }
        }
    };
    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '.' || b[i][j] == '#' {
                continue;
            }
            let k = b[i][j].to_digit(10).unwrap() as usize;
            bomb(i, j, k);
        }
    }

    for i in 0..r {
        for j in 0..c {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
