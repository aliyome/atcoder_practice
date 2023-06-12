use proconio::{input, marker::Chars};

fn main() {
    input! {
      _s: Chars,
      _t: Chars,
    }

    let mut s = vec![' '];
    s.extend(_s);
    let mut t = vec![' '];
    t.extend(_t);

    let mut dp = vec![vec![0; t.len()]; s.len()];
    for i in 0..s.len() {
        dp[i][0] = i;
    }
    for j in 0..t.len() {
        dp[0][j] = j;
    }
    for i in 1..s.len() {
        for j in 1..t.len() {
            // 削除
            let case1 = dp[i - 1][j] + 1;
            // 挿入
            let case2 = dp[i][j - 1] + 1;
            // 変更
            let change = if s[i] == t[j] { 0 } else { 1 };
            let case3 = dp[i - 1][j - 1] + change;
            dp[i][j] = case1.min(case2).min(case3);
        }
    }

    println!("{}", dp[s.len() - 1][t.len() - 1]);
}
