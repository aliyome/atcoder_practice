use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, isize); n]
    };

    // dp[i][j] := i番目までの料理を食べた時の最大値。j=0:お腹を壊していない、j=1:お腹を壊している
    let mut dp = vec![vec![0; 2]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = -10isize.pow(10);

    for i in 0..n {
        let (x, y) = xy[i];
        dp[i + 1][0] = dp[i][0];
        dp[i + 1][1] = dp[i][1];
        if x == 0 {
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + y);
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][1] + y);
        } else {
            dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + y);
        }
    }
    println!("{}", dp[n][0].max(dp[n][1]));
}
