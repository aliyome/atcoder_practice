use itertools::Itertools;
use proconio::input;

// bitDP
fn main() {
    input! {
        (n, k): (usize, usize),
        xy: [(i64, i64); n]
    }

    // dists[i][j] := i から j への距離の2乗
    let mut dists = vec![vec![0; n]; n];
    for (i, (x, y)) in xy.iter().enumerate() {
        for (j, (x2, y2)) in xy.iter().enumerate() {
            if i == j {
                continue;
            }
            let dist = (x - x2).pow(2) + (y - y2).pow(2);
            dists[i][j] = dist;
        }
    }

    // cost[b] := 頂点群 b (bitで表現) を1つのグループにする時のグループ内頂点間の最大距離
    let mut cost = vec![0; 1 << n];
    for i in 1..1 << n {
        for j in 0..n {
            // ↓ 0..n でも良いが、0..nとj..nは鏡像でなので不要な重複計算を避けるために 0..j とする
            for kk in 0..j {
                if (i >> j) & 1 == 1 && (i >> kk) & 1 == 1 {
                    cost[i] = cost[i].max(dists[j][kk])
                }
            }
        }
    }

    // dp[cnt][b] := すでに選んだ頂点群 b (bitで表現)で、cnt グループに分けた時のスコアの最小値
    // dp[cnt][b] = min max(dp[cnt - 1][b], d[bit - b])
    let mut dp = vec![vec![std::usize::MAX; 1 << n]; k + 1];
    dp[0][0] = 0;

    // クラスタ数を1つずつ増やす
    for i in 1..=k {
        // 全通りの頂点群を試す
        for j in 1..1 << n {
            // 頂点群 j から部分集合 b を選ぶ
            // 最初は b = j とする
            let mut b = j;
            while b != 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - b].max(cost[b] as usize));
                // jのすべての部分集合を試す
                b = (b - 1) & j;
            }
        }
    }

    println!("{}", dp[k][(1 << n) - 1])
}
