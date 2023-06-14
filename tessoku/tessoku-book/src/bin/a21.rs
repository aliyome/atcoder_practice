use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n]
    }

    // 1-indexed
    let mut pa = pa;
    pa.insert(0, (0, 0));
    pa.push((0, 0));

    // dp[l][r] := l..r が残っている時のコストの得点の最大値
    let mut dp = vec![vec![0; n + 2]; n + 2];
    // 全部残ってる場合の得点は0
    dp[1][n] = 0;

    // 端から1つずつ取り除いていく
    for len in (0..=n - 2).rev() {
        for l in 1..=n - len {
            let r = l + len;

            // 左端を取り除く場合、piが残ってたら得点に加算
            dp[l][r] = dp[l][r].max(dp[l - 1][r]);
            if l <= pa[l - 1].0 && pa[l - 1].0 <= r {
                dp[l][r] = dp[l][r].max(dp[l - 1][r] + pa[l - 1].1);
            }
            // 右端を取り除く場合
            dp[l][r] = dp[l][r].max(dp[l][r + 1]);
            if l <= pa[r + 1].0 && pa[r + 1].0 <= r {
                dp[l][r] = dp[l][r].max(dp[l][r + 1] + pa[r + 1].1);
            }
        }
    }

    // 結果
    let mut max = 0;
    for i in 0..=n {
        max = max.max(dp[i][i]);
    }
    println!("{}", max);
}
