use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
      mut ab: [(usize, usize); m]
    }

    ab.sort_by(|a, b| a.0.cmp(&b.0));

    // score[l][r] := lページからrページまでが一つの章だった場合のスコア
    let mut score = vec![vec![0; n + 1]; n + 1];
    // O(N^2 x M) -> O(300 x 300 x 50) -> O(4_500_000)
    for l in 1..n {
        for r in l + 1..=n {
            for &(a, b) in &ab {
                if l <= a && b <= r {
                    score[l][r] += 1;
                }
            }
        }
        // println!("{:?}", score[l]);
    }

    // dp[i][j] := i章目がjページで終わる場合のスコアの最大値
    let mut dp = vec![vec![-1_000_000; n + 1]; k + 1];
    dp[0][0] = 0;
    // println!("---");

    // O(KN^2) -> O(10 x 300 x 300) -> O(900_000)
    // 貰うDP

    // i-1章目まで確定していて
    for i in 1..=k {
        // i章目がjページで終わるとき
        for j in 1..=n {
            // i-1章目がeページで終わったなら
            for e in 0..j {
                // i章目がjページで終わる場合のスコアの最大値は
                // i-1章目がeページで終わった場合のスコアの最大値に
                // e+1ページからjページまでが一つの章だった場合のスコアを足したもの
                dp[i][j] = dp[i][j].max(dp[i - 1][e] + score[e + 1][j]);
            }
        }
        // println!("{:?}", dp[i]);
    }

    println!("{}", dp[k][n]);
}
