use proconio::input;

const MOD: usize = 998244353;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    };

    // dp[i][j] := i番目までの数列で、jが作れる通り数
    let m = 3000;
    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        let mut acc = vec![0; m + 2];
        for j in 0..=m {
            acc[j + 1] = acc[j] + dp[i][j];
            acc[j + 1] %= MOD;
        }
        for j in a[i]..=b[i] {
            dp[i + 1][j] = acc[j + 1];
            dp[i + 1][j] %= MOD;
        }
    }

    let mut ans = 0;
    for i in 0..=m {
        ans += dp[n][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
