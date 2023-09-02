use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize, y: usize,
        ab: [(usize, usize); n]
    };

    // dp[i][j][k] := i番目まで見てたこ焼きをj個、たいやきをk個買うのに必要な最小の回数
    let inf = std::usize::MAX;
    let mut dp = vec![vec![vec![inf; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;

    // i番目まで見て
    for i in 0..n {
        let (a, b) = ab[i];
        // たこ焼きをj個
        for j in 0..=x {
            // たいやきをk個
            for k in 0..=y {
                // 買えるなら
                if dp[i][j][k] != inf {
                    // 買わない場合
                    dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                    // 買う場合
                    let nj = (j + a).min(x);
                    let nk = (k + b).min(y);
                    dp[i + 1][nj][nk] = dp[i + 1][nj][nk].min(dp[i][j][k] + 1);
                }
            }
        }
    }

    if dp[n][x][y] == inf {
        println!("-1");
    } else {
        println!("{}", dp[n][x][y]);
    }
}
