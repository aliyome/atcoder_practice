use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    };

    // dp[i][j] := i番目まで見て、その時点の数列の合計がjであるような数列の個数
    let mut dp = vec![vec![0; n * m + 1]; n + 1];
    dp[0][0] = 1;

    // i番目まで見て
    for i in 0..n {
        // 現在の値がjで
        for j in 0..=k {
            // a足す
            for a in 1..=m {
                // kを超えたら終了
                if j + a > k {
                    break;
                }

                dp[i + 1][j + a] += dp[i][j];
                dp[i + 1][j + a] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for i in 1..=k {
        ans += dp[n][i];
        ans %= MOD;
    }

    println!("{}", ans);
}
