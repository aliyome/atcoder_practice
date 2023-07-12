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

    for i in 0..h {
        for j in 0..w {
            // 右
            for k in 1..=2000 {
                if j + k < w && s[i][j + k] == '.' {
                    dp[i][j + k] += dp[i][j];
                    dp[i][j + k] %= MOD;
                } else {
                    break;
                }
            }
            // 下
            for k in 1..=2000 {
                if i + k < h && s[i + k][j] == '.' {
                    dp[i + k][j] += dp[i][j];
                    dp[i + k][j] %= MOD;
                } else {
                    break;
                }
            }
            // 右下
            for k in 1..=2000 {
                if i + k < h && j + k < w && s[i + k][j + k] == '.' {
                    dp[i + k][j + k] += dp[i][j];
                    dp[i + k][j + k] %= MOD;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
