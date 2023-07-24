use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut holes = HashSet::new();
    for &(a, b) in &ab {
        holes.insert((a - 1, b - 1));
    }

    // dp[i][j] := (i, j) を左上頂点とする正方形の数
    let mut dp = vec![vec![0usize; w + 1]; h + 1];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if holes.contains(&(i, j)) {
                dp[i][j] = 0;
            } else {
                dp[i][j] = dp[i + 1][j].min(dp[i][j + 1]).min(dp[i + 1][j + 1]) + 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += dp[i][j];
        }
    }

    println!("{}", ans);
}
