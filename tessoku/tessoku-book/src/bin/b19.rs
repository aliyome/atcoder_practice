use proconio::input;

fn main() {
    input! {
      n: usize, // <= 100
      w: usize, // <= 10^9
      _wv: [(usize, usize); n] // v <= 1000
    }

    // 1-indexed
    let mut wv = vec![(0, 0)];
    wv.extend(_wv);
    wv.sort_by(|a, b| a.0.cmp(&b.0));

    // dp[i][j] := i番目までの品物を使ってjの価値を実現する時の重さの最小値
    let mut dp = vec![vec![std::usize::MAX; 1000 * 100 + 1]; n + 1];
    dp[0][0] = 0;

    // i番目の商品を使うかどうかで場合分け
    for i in 1..=n {
        let (weight, value) = wv[i];
        for j in 0..=1000 * 100 {
            // i番目の商品を使わない場合
            dp[i][j] = dp[i - 1][j];
            // i番目の商品を使う場合
            if j < value {
                // 価値が足りないので使えない
                continue;
            }
            // 到達不可能な場合はスキップ
            if dp[i - 1][j - value] == std::usize::MAX {
                continue;
            }
            dp[i][j] = dp[i][j].min(dp[i - 1][j - value] + weight);
        }
    }

    for v in (0..=1000 * 100).rev() {
        if dp[n][v] != std::usize::MAX && dp[n][v] <= w {
            println!("{}", v);
            return;
        }
    }
}
