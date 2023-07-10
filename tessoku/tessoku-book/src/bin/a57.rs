use proconio::input;

fn main() {
    input! {
      n: usize,
      q: usize,
      mut a: [usize; n],
      xy: [(usize, usize); q] // y <= 10^9
    }

    // 1-indexed
    a.insert(0, 0);

    // dp[d][i] := 穴iに居たアリの2^d日後の場所
    // 条件(y <= 10^9) より、 d <= 30 (10^9 < 2^30)
    let mut dp = vec![vec![0; n + 1]; 30 + 1];
    // 初期値 dp[0][i] := 穴iに居たアリの1日後の場所 → 問題文より a[i]
    for i in 1..=n {
        dp[0][i] = a[i];
    }
    // dp[d][i] := dp[d - 1][dp[d - 1][i]] という漸化式が成り立つ
    // ex: dp[1][i] = dp[0][ dp[0][i] ]
    for d in 0..30 {
        for i in 1..=n {
            dp[d + 1][i] = dp[d][dp[d][i]];
        }
    }

    for &(x, y) in &xy {
        let mut x = x;
        let mut y = y;
        while y > 0 {
            // let d = y.ilog2() as usize; // Rust 1.67 から使える
            let d = (y as f64).log2().floor() as usize;
            x = dp[d][x];
            y = y - 2usize.pow(d as u32);
            // println!("d:{} x:{} left:{}", d, x, y);
        }
        println!("{}", x);
    }
}
