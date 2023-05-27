use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: i64, y: i64, z: i64,
        s: Chars,
    }

    let n = s.len();
    // dp[i][j]: i 文字目まで見たあとに caps が j であるときの最小コスト
    // caps: 0 -> off, 1 -> on
    let mut dp = vec![vec![1 << 60; 2]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = z;

    for i in 0..n {
        if s[i] == 'a' {
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + x).min(dp[i][1] + z + y);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + y).min(dp[i][0] + z + x);
        } else {
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + y).min(dp[i][1] + z + x);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + x).min(dp[i][0] + z + y);
        }
    }

    println!("{}", dp[n][0].min(dp[n][1]));
}
