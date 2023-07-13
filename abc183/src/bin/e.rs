use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

// TLE
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };

    // dp[i][j] = (i, j) に到達する場合の数
    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[0][0] = 1;

    let mut x = vec![vec![0; w + 1]; h + 1];
    let mut y = vec![vec![0; w + 1]; h + 1];
    let mut z = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            if s[i][j] == '#' {
                continue;
            }
            // 右
            if 0 < j {
                x[i][j] = x[i][j - 1] + dp[i][j - 1];
                x[i][j] %= MOD;
            }
            // 下
            if 0 < i {
                y[i][j] = y[i - 1][j] + dp[i - 1][j];
                y[i][j] %= MOD;
            }
            // 右下
            if 0 < i && 0 < j {
                z[i][j] = z[i - 1][j - 1] + dp[i - 1][j - 1];
                z[i][j] %= MOD;
            }
            dp[i][j] += x[i][j] + y[i][j] + z[i][j];
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
