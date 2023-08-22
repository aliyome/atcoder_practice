use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        mut s: Chars
    }

    let atcoder = "atcoder".chars().collect::<Vec<_>>();

    // dp[i][j] := i文字目まで見た時に、j文字目までの文字列を作る場合の数
    let mut dp = vec![vec![0; atcoder.len() + 1]; n + 1];
    dp[0][0] = 1;

    // 配るDP
    for i in 0..n {
        dp[i + 1] = dp[i].clone();
        for j in 0..atcoder.len() {
            if s[i] == atcoder[j] {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= MOD;
            }
        }
    }

    println!("{}", dp[n][atcoder.len()]);
}
