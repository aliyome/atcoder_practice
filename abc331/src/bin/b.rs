use proconio::input;

fn main() {
    input! {
        n: usize,
        s: isize,
        m: isize,
        l: isize
    };

    // dp[i] := i 個目の卵を買ったときの最小の金額
    let inf = 10isize.pow(15);
    let mut dp = vec![inf; n + 50];
    dp[0] = 0;
    for i in 0..=n + 12 {
        if dp[i] == inf {
            continue;
        }
        dp[i + 6] = dp[i + 6].min(dp[i] + s);
        dp[i + 8] = dp[i + 8].min(dp[i] + m);
        dp[i + 12] = dp[i + 12].min(dp[i] + l);
    }

    let mut ans = inf;
    for i in n..=n + 12 {
        ans = ans.min(dp[i]);
    }

    println!("{}", ans);
}
