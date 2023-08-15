use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    // let mut ans = 0;
    // for i in 0..n {
    //     for j in (i + 1..=(n - i)).rev() {
    //         ans += j;
    //         ans %= MOD;
    //     }
    // }

    // dp[i][j] := [i, j) が条件を満たす場合の数
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in i..n {
            dp[i][j] += 1;
        }
    }

    println!("{}", ans);
}
