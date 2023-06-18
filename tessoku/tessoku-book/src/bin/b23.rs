use proconio::input;

fn main() {
    input! {
      n: usize,
      xy: [(f64, f64); n]
    }

    let mut dp = vec![vec![10.0f64.powf(10.0); n]; 1 << n];
    dp[0][0] = 0.0;

    for i in 0..1 << n {
        for j in 0..n {
            if dp[i][j] >= 10.0f64.powf(10.0) {
                continue;
            }
            for k in 0..n {
                // if (i & (1 << k)) != 0 {
                if i & (1 << k) != 0 {
                    continue;
                }
                let d = ((xy[j].0 - xy[k].0).powf(2.0) + (xy[j].1 - xy[k].1).powf(2.0)).sqrt();
                dp[i | (1 << k)][k] = dp[i | (1 << k)][k].min(dp[i][j] + d);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1][0]);
}
