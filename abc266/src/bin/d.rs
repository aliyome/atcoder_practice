use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, isize); n]
    }

    let max_t = txa.iter().map(|&(t, _, _)| t).max().unwrap();

    let mut map = HashMap::new();
    for &(t, x, a) in &txa {
        map.insert(t, (x, a));
    }

    // dp[t][j] := 時刻tにおいて、穴jに居るときの最大スコア
    let mut dp = vec![vec![-10isize.pow(18); 5]; 100_001];
    dp[0][0] = 0;

    // もらうDP
    for t in 1..=100_000 {
        for j in 0..5 {
            if j > 0 {
                dp[t][j] = dp[t][j].max(dp[t - 1][j - 1]);
            }
            dp[t][j] = dp[t][j].max(dp[t - 1][j]);
            if j < 4 {
                dp[t][j] = dp[t][j].max(dp[t - 1][j + 1]);
            }
        }
        if let Some(&(x, a)) = map.get(&t) {
            dp[t][x] += a;
        }
    }

    println!("{}", dp[100_000].iter().max().unwrap());
}
