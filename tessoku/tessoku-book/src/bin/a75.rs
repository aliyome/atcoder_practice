use proconio::input;

fn main() {
    input! {
      n: usize,
      mut td: [(usize, usize); n]
    }

    // 締め切りが早い順にソート
    td.sort_by(|a, b| a.1.cmp(&b.1));
    // 1-indexed
    td.insert(0, (0, 0));

    // dp[i][j] := i問目を解いた時点で現在時刻がjの時、最大何問解けるか
    let time_limit = 1440;
    let mut dp = vec![vec![0; time_limit + 1]; n + 1];

    // 貰うDP
    for i in 1..=n {
        let (t, d) = td[i];
        for j in 0..=time_limit {
            if j > d || j < t {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - t] + 1);
            }
        }
    }

    let mut max = 0;
    for i in 0..=time_limit {
        max = max.max(dp[n][i]);
    }

    println!("{}", max);
}
