use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        l: usize
    }

    // dp[i] := i 段目にたどり着くまでの移動方法の数
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 0..=n {
        if i + 1 <= n {
            dp[i + 1] = (dp[i + 1] + dp[i]) % MOD;
        }
        if i + l <= n {
            dp[i + l] = (dp[i + l] + dp[i]) % MOD;
        }
    }

    println!("{}", dp[n]);
}
