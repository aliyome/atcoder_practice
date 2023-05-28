use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        mut lrv: [(i64, i64, i64); n],
    }

    // l,rでソート
    let mut dishes = vec![];
    dishes.push((0, 0, 0)); // 1-indexedにするために0を追加
    dishes.extend(lrv);

    // dp[i][j] = i番目までの料理を使ってj[g]の香辛料を使うときの価値の最大値
    // dp[i][j] は以下の max
    // 1. i番目の料理を使わない場合 -> dp[i-1][j]
    // 2. i番目の料理を使う場合 -> dp[i-1][j - k] + v (k = [l,r])
    // 計算量 O(NW^2)
    let mut dp = vec![vec![std::i64::MIN; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let (l, r, v) = dishes[i];
        for j in 0..=w {
            // i番目の料理を使わない場合はi-1番目までと同じ
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            // i番目の料理を使う場合
            for k in l..=r {
                if j as i64 - k < 0 {
                    continue;
                }
                let k = k as usize;
                dp[i][j] = dp[i][j].max(dp[i - 1][j - k] + v);
            }
        }
    }

    println!("{}", dp[n][w].max(-1));
}
