use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut dp = vec![std::usize::MAX; 100011];
    dp[0] = 0;

    for i in 0..=100000 {
        if dp[i] == std::usize::MAX {
            continue;
        }
        dp[i + 1] = dp[i + 1].min(dp[i] + 1);

        let mut j = 6;
        while i + j <= 100000 {
            dp[i + j] = dp[i + j].min(dp[i] + 1);
            j *= 6;
        }

        let mut j = 9;
        while i + j <= 100000 {
            dp[i + j] = dp[i + j].min(dp[i] + 1);
            j *= 9;
        }
    }

    println!("{}", dp[n]);
}
