use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n - 1], // a[i] に進み、スコア100をえる
      mut b: [usize; n - 1], // b[i] に進み、スコア150をえる
    }

    // edges[i] := iマス目が遷移先のマスリスト
    let mut edges_a = vec![vec![]; n + 1];
    let mut edges_b = vec![vec![]; n + 1];
    for i in 0..n - 1 {
        edges_a[a[i]].push(i + 1);
        edges_b[b[i]].push(i + 1);
    }

    // dp[i] := iマス目にいるときの最大スコア
    let mut dp = vec![0; n + 1];
    dp[1] = 0;
    for i in 2..=n {
        // iマス目には edges_a[i], edges_b[i] からしか遷移できない
        for &a in edges_a[i].iter() {
            dp[i] = dp[i].max(dp[a] + 100);
        }
        for &b in edges_b[i].iter() {
            dp[i] = dp[i].max(dp[b] + 150);
        }
    }

    println!("{}", dp[n]);
}
