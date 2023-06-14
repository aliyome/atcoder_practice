use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n - 1],
      mut b: [usize; n - 2]
    }

    // // A16 貰うDP
    // let mut dp = vec![100000000; n + 1];
    // dp[0] = 0;
    // for i in 1..n {
    //     dp[i] = dp[i].min(dp[i - 1] + a[i - 1]);
    //     if i >= 2 {
    //         dp[i] = dp[i].min(dp[i - 2] + b[i - 2]);
    //     }
    // }

    // println!("{}", dp[n - 1]);

    // B22 配るDP
    // 1-indexed
    a.insert(0, 0);
    a.insert(0, 0);
    b.insert(0, 0);
    b.insert(0, 0);
    b.insert(0, 0);

    let mut dp = vec![100000000; n + 1];
    dp[1] = 0;
    for i in 1..=n {
        if i <= n - 1 {
            dp[i + 1] = dp[i + 1].min(dp[i] + a[i + 1]);
        }
        if i <= n - 2 {
            dp[i + 2] = dp[i + 2].min(dp[i] + b[i + 2]);
        }
    }

    println!("{}", dp[n]);
}
