use proconio::{input, marker::Chars};

const MOD: usize = 1000000007;

fn main() {
    input! {
        s: Chars,
    };

    let t = "chokudai".chars().collect::<Vec<char>>();
    let n = s.len();

    let mut dp = vec![vec![0 as usize; n + 1]; 9];
    for j in 0..=n {
        dp[0][j] = 1;
    }

    for i in 0..8 {
        for j in 0..n {
            if s[j] == t[i] {
                dp[i + 1][j + 1] = (dp[i][j] + dp[i + 1][j]) % MOD;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j];
            }
        }
    }

    println!("{}", dp[8][n]);
}
