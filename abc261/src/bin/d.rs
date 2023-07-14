use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [u64; n],
        cy: [(u64, u64); m],
    };

    // // 1-indexed
    // x.insert(0, 0);

    let mut bonus = HashMap::new();
    for (c, y) in cy {
        bonus.insert(c, y);
    }

    // dp[i][j] := i番目までの時点で連続jだったときの最大報酬
    let mut dp = vec![vec![0u64; 5002]; n + 1];
    dp[0][0] = 0;

    // 配るDP
    for i in 0..n {
        for j in 0..=5000 {
            // 試行回数が少なすぎて連続になり得ない場合
            if i < j {
                continue;
            }
            // 裏の場合
            dp[i + 1][0] = dp[i][j];
            // 表の場合
            // 通常報酬
            let mut next = dp[i][j] + x[i];
            // ボーナス報酬（あれば）
            next += bonus.get(&(j as u64 + 1)).unwrap_or(&0);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(next)
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
