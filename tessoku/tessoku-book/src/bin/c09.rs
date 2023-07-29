use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n]
    }

    // 1-indexed
    a.insert(0, 0);

    // dp[i] := i日目までの現在の最大値
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = a[1];
    // 貰うDP
    for i in 2..=n {
        dp[i] = dp[i - 1].max(dp[i - 2] + a[i]);
    }

    println!("{}", dp[n]);
}
