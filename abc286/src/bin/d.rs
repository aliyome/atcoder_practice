use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut ab: [(usize, usize); n],
    }

    // コインを小さい順にソート
    ab.sort_by_key(|&(_, b)| Reverse(b));

    // コインを枚数分展開する
    let mut coins = vec![];
    for (a, b) in ab {
        for _ in 0..b {
            coins.push((a, b));
        }
    }

    // DP
    // dp[i][j]: i番目までのコインを使ってj円を作れるかどうか
    let mut dp = vec![vec![false; x + 1]; coins.len() + 1];
    dp[0][0] = true;

    // i番目のコインを使うかどうか
    for i in 0..coins.len() {
        // j 円を作れるか
        for j in 0..=x {
            // 一つ前の状態で作れない場合はi番目のコインを使っても作れない。
            // ※i番目のコインはi番目より前のコインより金額が大きいため
            if !dp[i][j] {
                continue;
            }
            // i番目のコインを使わない場合はi-1番目までのコインを使ってj円を作れるかどうかと同じ結果
            dp[i + 1][j] = dp[i][j];

            // i番目のコインを使う場合
            if j + coins[i].0 > x {
                // X円を超えるなら使えない
                continue;
            }
            // j + coins[i].0 円を作れる
            dp[i + 1][j + coins[i].0] = true;
        }
    }

    if dp[coins.len()][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
