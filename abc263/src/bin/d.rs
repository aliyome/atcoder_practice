use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n]
    };

    // dp[i] := 要素iまで見た時に、最後に選んだ要素がjの時の最小値。j := 0:L, 1:A, 2:R
    let mut dp = vec![vec![std::isize::MAX; 3]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    dp[0][2] = 0;
    for i in 0..n {
        // l を使う場合
        dp[i + 1][0] = dp[i + 1][0].min(dp[i][0].saturating_add(l));
        // a を使う場合
        dp[i + 1][1] = dp[i + 1][1]
            .min(dp[i][0].saturating_add(a[i]))
            .min(dp[i][1].saturating_add(a[i]));
        // r を使う場合
        dp[i + 1][2] = dp[i + 1][2]
            .min(dp[i][0].saturating_add(r))
            .min(dp[i][1].saturating_add(r))
            .min(dp[i][2].saturating_add(r));
    }
    println!("{}", dp[n][0].min(dp[n][1]).min(dp[n][2]));
}
