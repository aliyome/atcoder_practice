use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    // dp[i][j] := 上からi段目のj番目のマスに到達するまでの最適解
    let mut dp = vec![vec![std::usize::MAX; n]; n];

    // 最下段
    for j in 0..n {
        dp[n - 1][j] = a[j];
    }

    for i in (1..n).rev() {
        for j in 0..i {
            if i % 2 == 0 {
                dp[i - 1][j] = dp[i][j].min(dp[i][j + 1]);
            } else {
                dp[i - 1][j] = dp[i][j].max(dp[i][j + 1]);
            }
        }
    }

    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

    println!("{}", dp[0][0]);
}
