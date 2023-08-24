use std::collections::HashSet;

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    };

    let mut set = HashSet::new();
    for &a in &a {
        set.insert(a);
    }

    let mut dp = vec![0; n + 5];
    dp[0] = 1;

    for i in 0..n {
        if !set.contains(&(i + 1)) {
            dp[i + 1] += dp[i];
            dp[i + 1] %= MOD;
        }
        if !set.contains(&(i + 2)) {
            dp[i + 2] += dp[i];
            dp[i + 2] %= MOD;
        }
    }

    println!("{}", dp[n]);
}
