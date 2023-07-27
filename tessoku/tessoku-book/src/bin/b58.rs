use proconio::input;

fn main() {
    input! {
      n: usize, // <= 10^5
      l: usize,
      r: usize,
      mut x: [usize; n]
    }

    // 配るDP
    let mut dp = vec![std::usize::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        // TODO: 二分探索
        for j in i + 1..n {
            if x[i] + l <= x[j] && x[j] <= x[i] + r {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }
    }
    println!("{}", dp[n - 1]);
}
