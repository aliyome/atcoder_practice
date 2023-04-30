use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        s: usize,
    };

    let mut dp = vec![0; s + 1];
    dp[0] = 1;

    for i in 3..=s {
        dp[i] = (dp[i] + dp[i - 3]) % MOD;
    }

    println!("{}", dp[s]);
}
