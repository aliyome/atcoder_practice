use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
    };

    // dp[j][i] = i - 1 桁の数で、j が最後になるもので条件を満たす数
    let mut dp = vec![vec![0; n + 1]; 11];

    // 1 桁の数は 1 から 9 までそれぞれが条件を満たす
    for j in 1..=9 {
        dp[j][0] = 1;
    }

    // 2桁目以降はDPで求める
    for i in 1..=n {
        for j in 1..=9 {
            // 1桁前の数が j とその前後の数 の個数を足す
            dp[j][i] += dp[j - 1][i - 1];
            dp[j][i] += dp[j][i - 1];
            dp[j][i] += dp[j + 1][i - 1];
            dp[j][i] %= MOD;
        }
    }

    let mut ans = 0;
    for j in 1..=9 {
        ans += dp[j][n - 1];
        ans %= MOD;
    }

    println!("{}", ans);
}
