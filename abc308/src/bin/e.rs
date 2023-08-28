use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    // dp[i][j][k] := Siまでみて、MEXのうちj文字目まで完成しているとき、Aの集合をビットkで表現したときの総和
    let mut dp = vec![vec![; 4]; n + 1];
    for i in 0..n {
        // s[i]を選ばない
        for j in 0..=3 {
            dp[i + 1][j] = dp[i][j];
        }
        // s[i]を選ぶ
        if s[i] == 'M' {
            dp[i + 1][0] += 1;
        } else if s[i] == 'E' {
            dp[i + 1][1] += dp[i][0];
        } else {
            dp[i + 1][2] += dp[i][1];
        }
    }

    println!("{}", dp[n][2]);
}
