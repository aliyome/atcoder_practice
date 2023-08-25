use proconio::input;

const MOD: usize = 998244353;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    };

    // dp[i][j] := i番目の要素まで見たときに、末尾がjであるような条件を満たす数列の個数
    let mut dp = vec![vec![0; 3001]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..=3000 {
            for k in a[i].max(j)..=b[i] {
                dp[i + 1][k] += dp[i][j];
                dp[i + 1][k] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for j in 0..=3000 {
        ans += dp[n][j];
        ans %= MOD;
    }

    println!("{}", ans);
}
