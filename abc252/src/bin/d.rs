use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // Aiの値毎にグループ分けして数を数える
    let mut counts = HashMap::new();
    for e in a {
        *counts.entry(e).or_insert(0) += 1usize;
    }
    let counts = counts.values().collect::<Vec<_>>();

    // dp[i][j] = i番目までのグループを使ってj個の数列を作る場合の数
    let mut dp = vec![vec![0 as usize; 4]; 2 * 10usize.pow(5) + 1];
    dp[0][0] = 1;
    for j in 0..=3 {
        for i in 0..counts.len() {
            if j == 0 {
                dp[i + 1][j] = 1;
            } else {
                dp[i + 1][j] = dp[i][j] + dp[i][j - 1] * counts[i];
            }
        }
    }

    println!("{}", dp[counts.len()][3]);
}
