use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let m = "atcoder".len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    dp[0] = vec![1; n + 1];

    let t: Vec<char> = "atcoder".chars().collect();
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] += dp[i][j - 1] % 1_000_000_007;
            if t[i - 1] == s[j - 1] {
                dp[i][j] += dp[i - 1][j - 1] % 1_000_000_007;
            }
        }
        // println!("{:?}", dp[i]);
    }

    println!("{}", dp[m][n]);
}
