use proconio::input;

const MOD: isize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    // dp[i][j][k] := ai まで見て、 j 個選んで、和が k になる場合の数 -> k が取りうる範囲は 10^9 x 100 なので出来ない
    // let mut dp = vec![vec![vec![0; 10^9 * 100 + 1]; n + 1]; n + 1];

    let mut ans = 0;
    // mで割った時のあまり
    for m in 1..=n {
        // dp[i][j][k] := ai まで見て、 j 個選んで、 和を m[1,n] で割った時のあまりが k となる場合の数 -> k が取りうる範囲は 100 なので 100^4 = 10^8 できる
        let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
        // 0個選んで和が0になる場合の数は1
        dp[0][0][0] = 1;

        // aiまで見て
        for i in 0..n {
            // aiを選ばない場合は dp[i+1] と dp[i] は同じ
            dp[i + 1] = dp[i].clone();

            // j個選んで
            for j in 0..n {
                // その和を m で割った時のあまりが k になる時
                for k in 0..=n {
                    // 場合の数が 0 の場合はスキップ
                    if dp[i][j][k] == 0 {
                        continue;
                    }

                    // aiを選ぶ場合
                    let nk = (k + a[i]) % m;
                    dp[i + 1][j + 1][nk] += dp[i][j][k];
                    dp[i + 1][j + 1][nk] %= MOD;
                }
            }
        }
        ans += dp[n][m][0];
        ans %= MOD;
    }

    println!("{}", ans);
}
