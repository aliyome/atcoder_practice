use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        mut ab: [(usize, usize); n],
    };

    // 効率が良い順、効率が同じなら消費MPが少ない順にソート
    ab.sort_by(|a, b| (b.0 * a.1).cmp(&(a.0 * b.1)).then(a.1.cmp(&b.1)));

    // dp[i][j] := i番目までの魔法を使ってjダメージを与えるのに必要なMP
    let mut dp = vec![vec![100_000_000; 30_000]; n + 1];
    dp[0][0] = 0;

    // 貰うDP
    for i in 1..n {
        let (a, b) = ab[i - 1];
        for j in 0..=h {
            let tj = (j as isize - a as isize).max(0) as usize;
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            dp[i][j] = dp[i][j].min(dp[i - 1][tj] + b);
            dp[i][j] = dp[i][j].min(dp[i][tj] + b);
        }
    }

    println!("{}", dp[n - 1][h]);
}
