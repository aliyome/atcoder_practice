use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, isize); n]
    };

    // dp[i][j] := i 番目までの料理を食べた時に、最後に食べた後の状態が j の時の美味しさの総和
    //             j=0: 正常, j=1: 異常
    let mut dp = vec![vec![0isize; 2]; n + 1];
    dp[0][1] = -10isize.pow(10);
    dp[0][0] = 0;

    for i in 0..n {
        let (x, y) = xy[i];
        // 食べない場合
        dp[i + 1][0] = dp[i][0];
        dp[i + 1][1] = dp[i][1];

        // 食べる場合
        if x == 0 {
            // 解毒
            // good -> good
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + y);
            // bad -> good
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][1] + y);
        } else {
            // 毒
            // good -> bad
            dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + y);
            // bad -> die
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
