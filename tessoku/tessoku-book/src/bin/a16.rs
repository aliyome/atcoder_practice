use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n - 1],
      b: [usize; n - 2]
    }

    let mut dp = vec![100000000; n + 1];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = dp[i].min(dp[i - 1] + a[i - 1]);
        if i >= 2 {
            dp[i] = dp[i].min(dp[i - 2] + b[i - 2]);
        }
    }

    println!("{}", dp[n - 1]);
}
