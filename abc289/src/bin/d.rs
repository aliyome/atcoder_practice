use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize
    };

    let mut mochi = HashSet::new();
    for &b in &b {
        mochi.insert(b);
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for i in 0..x {
        for j in 0..n {
            if !dp[i] {
                continue;
            }
            if i + a[j] <= x && !mochi.contains(&(i + a[j])) {
                dp[i + a[j]] = true;
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
