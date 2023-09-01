use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for j in 1..=m {
        dp[1][j] = 1;
    }

    // iまでみて
    for i in 2..=n {
        // i-1の累積和
        let mut acc = vec![0; m + 1];
        for j in 1..=m {
            acc[j] = (acc[j - 1] + dp[i - 1][j]) % MOD;
        }

        // 次がjの時
        for j in 1..=m {
            let mut x = 0;
            let mut y = 0;
            if k == 0 {
                dp[i][j] += acc[m];
                dp[i][j] %= MOD;
            } else {
                // [1, j - k]までの累積和
                if j >= k {
                    x = acc[j - k];
                }
                // [j + k, m]までの累積和
                if j + k <= m {
                    y = (MOD + acc[m] - acc[j + k - 1]) % MOD;
                }
                dp[i][j] = (x + y) % MOD;
            }
        }
    }

    let mut ans = 0;
    for j in 1..=m {
        ans = (ans + dp[n][j]) % MOD;
    }
    println!("{}", ans);
}
