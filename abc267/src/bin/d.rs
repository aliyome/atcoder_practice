use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n]
    };

    // 1-indexed
    a.insert(0, 0);

    // dp[i][j] := i番目まで見たときの長さjの部分列の最大値
    let mut dp = vec![vec![-10isize.pow(18); m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        dp[i] = dp[i - 1].clone();
        for j in 1..=m {
            dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + j as isize * a[i]);
        }
    }

    println!("{}", dp[n][m]);
}
