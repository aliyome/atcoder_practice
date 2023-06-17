use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

// 桁DP
// a+b == a^b -> a&b == 0
fn main() {
    input! {
        l: Chars,
    };

    // 桁数
    let n = l.len();

    // dp[i][j] := 先頭からi桁目まで見たときに以下の条件を満たす組み合わせの個数
    //             j=0: i桁目までLと一致している
    //             j=1: i桁目までの数が l 未満であることが確定している
    let mut dp = vec![vec![0; 2]; n + 1];
    dp[0][0] = 1;

    // 先頭からi桁目までを見て
    for i in 0..n {
        if l[i] == '0' {
            dp[i + 1][0] = dp[i][0];
            dp[i + 1][1] = (dp[i][1] * 3) % MOD;
        } else {
            dp[i + 1][0] = (dp[i][0] * 2) % MOD;
            dp[i + 1][1] = (dp[i][1] * 3) % MOD;
            dp[i + 1][1] += dp[i][0] % MOD;
        }
    }

    let ans = (dp[n][0] + dp[n][1]) % MOD;
    println!("{}", ans);
}
