use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      s: Chars
    }

    // dp[l][r] := s[l..r] の最長回文部分列の長さ
    let mut dp = vec![vec![0; n]; n];
    // 初期値の設定
    // 文字列の長さが1なら回文部分列の長さは常に1
    // 文字列の長さが2なら回文部分列の長さは1か2
    for i in 0..n {
        dp[i][i] = 1;
    }
    for i in 0..n - 1 {
        dp[i][i + 1] = if s[i + 1] == s[i] { 2 } else { 1 };
    }

    // 長さ3〜Nまでを順番に求めていく
    for len in 2..n {
        for l in 0..n - len {
            let r = l + len;
            if n <= r {
                continue;
            }
            // 回文が拡張されない場合は一文字少ない回文の長さを引き継ぐ
            dp[l][r] = dp[l][r].max(dp[l + 1][r]).max(dp[l][r - 1]);
            // 回分が拡張される場合は回文の長さに2を足す
            if s[l] == s[r] {
                dp[l][r] = dp[l][r].max(dp[l + 1][r - 1] + 2);
            }
        }
    }

    for i in 0..n {
        println!("{:?}", dp[i]);
    }

    println!("{:?}", dp[0][n - 1]);
}
