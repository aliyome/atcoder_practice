use proconio::input;

fn main() {
    input! {
      n: usize,
      h: [isize; n]
    }

    let mut dp = vec![100000000; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        dp[i] = dp[i].min(dp[i - 1] + (h[i] - h[i - 1]).abs());
        dp[i] = dp[i].min(dp[i - 2] + (h[i] - h[i - 2]).abs());
    }

    println!("{}", dp[n - 1]);
}
