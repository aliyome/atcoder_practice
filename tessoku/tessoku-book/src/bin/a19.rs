use proconio::input;

fn main() {
    input! {
      n: usize,
      w: usize,
      mut _wv: [(usize, isize); n],
    }

    // 軽い順にソート
    _wv.sort_by(|a, b| a.0.cmp(&b.0));
    // 1-indexed
    let mut wv = vec![(10usize.pow(9), 0)];
    wv.extend(&_wv);

    // dp[i][j] := i番目までの品物を使って重さjを作った時の最大価値
    let mut dp = vec![vec![-100; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        let (weight, value) = wv[i];
        for j in 0..=w {
            dp[i][j] = dp[i - 1][j];
            if weight <= j {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - weight] + value);
            }
        }
    }

    let mut max = 0;
    for &v in &dp[n] {
        max = max.max(v);
    }

    println!("{}", max);
}
