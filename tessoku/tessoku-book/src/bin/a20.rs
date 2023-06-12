use proconio::{input, marker::Chars};

fn main() {
    input! {
      _s: Chars,
      _t: Chars
    }

    // 1-indexed
    let mut s = vec!['@'];
    s.extend(_s);
    let mut t = vec!['@'];
    t.extend(_t);

    // dp[i][j] := s の i 文字目までと t の j 文字目までの LCS の長さ
    let mut dp = vec![vec![0; t.len()]; s.len()];

    for i in 1..s.len() {
        for j in 1..t.len() {
            // i - 1 引き継ぎ
            dp[i][j] = dp[i - 1][j];
            // j - 1 引き継ぎ
            dp[i][j] = dp[i][j].max(dp[i][j - 1]);
            // 連結成分追加
            if s[i] == t[j] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
        }
    }

    let mut max = 0;
    for j in 1..t.len() {
        max = max.max(dp[s.len() - 1][j]);
    }

    println!("{}", max);
}
