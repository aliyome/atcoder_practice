use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    // dp[i][j] := i枚目まで見た時に、j=0ならAを、j=1ならBを選んだ時の場合の数
    let mut dp = vec![vec![0; 2]; n + 1];
    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 1..n {
        if ab[i].0 != ab[i - 1].0 {
            dp[i][0] += dp[i - 1][0];
        }
        if ab[i].0 != ab[i - 1].1 {
            dp[i][0] += dp[i - 1][1];
        }
        if ab[i].1 != ab[i - 1].0 {
            dp[i][1] += dp[i - 1][0];
        }
        if ab[i].1 != ab[i - 1].1 {
            dp[i][1] += dp[i - 1][1];
        }
        dp[i][0] %= 998244353;
        dp[i][1] %= 998244353;
    }

    println!("{}", (dp[n - 1][0] + dp[n - 1][1]) % 998244353);
}
