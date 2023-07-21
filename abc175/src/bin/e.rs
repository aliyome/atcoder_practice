use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(usize, usize, usize); k]
    };

    let mut item_map = HashMap::new();
    for &(r, c, v) in &rcv {
        item_map.insert((r, c), v);
    }

    // dp[a][i][j] := i行j列に居る時に、
    //                その行ですでにa個のアイテムを拾っている時の、
    //                価値の最大値

    // 配るDP
    let mut dp = vec![vec![vec![0; c + 1]; r + 1]; 4];
    for i in 0..=r {
        for j in 0..=c {
            for a in 0..=3 {
                // 右に進む場合
                if j + 1 <= c {
                    dp[a][i][j + 1] = dp[a][i][j + 1].max(dp[a][i][j]);
                    if a < 3 {
                        if let Some(&v) = item_map.get(&(i, j + 1)) {
                            dp[a + 1][i][j + 1] = dp[a + 1][i][j + 1].max(dp[a][i][j] + v);
                        }
                    }
                }
                // 下に進む場合
                if i + 1 <= r {
                    dp[0][i + 1][j] = dp[0][i + 1][j].max(dp[a][i][j]);
                    if let Some(&v) = item_map.get(&(i + 1, j)) {
                        dp[1][i + 1][j] = dp[1][i + 1][j].max(dp[a][i][j] + v);
                    }
                }
            }
        }
    }

    let mut max = 0;
    for a in 0..=3 {
        max = max.max(dp[a][r][c]);
    }
    println!("{}", max);
}
