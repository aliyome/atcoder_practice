use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      a:[[usize; n]; m]
    }

    let mut bit_a = vec![0; m];
    for i in 0..m {
        for j in 0..n {
            bit_a[i] |= a[i][j] << (n - j - 1);
        }
    }
    // println!("{:?}", bit_a);

    // dp[i][j] := i番目までのクーポンを使ってj(bitマスク)の商品が買うときの必要枚数の最小値
    let mut dp = vec![vec![101; 1 << n + 1]; m + 1];
    dp[0][0] = 0;
    // i番目までのクーポンを使って
    for i in 0..m {
        // i+1番目はi番目の状態を引き継ぐ
        dp[i + 1] = dp[i].clone();

        // j(bitマスク)の商品を買うとき
        let a = &bit_a[i];
        for j in 0..1 << n {
            dp[i + 1][j | a] = dp[i + 1][j | a].min(dp[i][j] + 1);
        }
    }

    // for i in 0..=m {
    //     println!("{:?}", dp[i]);
    // }

    if dp[m][(1 << n) - 1] == 101 {
        println!("-1");
    } else {
        println!("{}", dp[m][(1 << n) - 1]);
    }
}
