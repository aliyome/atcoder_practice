use proconio::input;

fn main() {
    input! {
      n: usize,
      aa: [usize; n - 1],
      bb: [usize; n - 2]
    }

    // 2-indexed
    let mut a = vec![];
    a.push(0);
    a.push(0);
    a.extend(aa);

    // 3-indexed
    let mut b = vec![];
    b.push(0);
    b.push(0);
    b.push(0);
    b.extend(bb);

    let mut dp = vec![10000; n + 1];
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = a[2];
    for i in 3..=n {
        dp[i] = dp[i].min(dp[i - 1] + a[i]);
        dp[i] = dp[i].min(dp[i - 2] + b[i]);
    }

    println!("{}", dp[n]);
}
