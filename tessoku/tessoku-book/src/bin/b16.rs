use proconio::input;

fn main() {
    input! {
      n: usize,
      hh: [isize; n]
    }

    // 1-indexed
    let mut h = vec![0];
    h.extend(&hh);

    let mut dp = vec![100000000; n + 1];
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = (h[2] - h[1]).abs();
    for i in 3..=n {
        dp[i] = (dp[i - 1] + (h[i] - h[i - 1]).abs()).min(dp[i - 2] + (h[i] - h[i - 2]).abs());
    }

    println!("{}", dp[n]);
}
