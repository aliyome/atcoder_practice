use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 998244353;

fn main() {
    input! { s: Chars }
    let n = s.len();

    // dp[i][j] := i 文字目まで見たときに、開き括弧が j 個余っているような文字列の個数
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=i {
            if s[i] == '(' || s[i] == '?' {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= MOD;
            }
            if s[i] == ')' || s[i] == '?' {
                if j == 0 {
                    continue;
                }
                dp[i + 1][j - 1] += dp[i][j];
                dp[i + 1][j - 1] %= MOD;
            }
        }
    }

    // let mut max = 0;
    // for i in 1..=n {
    //     max = max.max(dp[i][0]);
    // }
    // println!("{}", max);
    println!("{}", dp[n][0]);
}
