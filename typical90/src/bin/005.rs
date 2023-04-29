use proconio::input;

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k]
    };

    const MOD: usize = 1_000_000_007;
    let mut dp = vec![vec![0; b]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..b {
            for &x in &c {
                dp[i + 1][(j * 10 + x) % b] += dp[i][j];
                dp[i + 1][(j * 10 + x) % b] %= MOD;
            }
        }
    }
    println!("{}", dp[n][0]);
}
