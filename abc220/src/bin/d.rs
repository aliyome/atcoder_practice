use std::collections::VecDeque;

use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut queue = VecDeque::new();
    for &x in a.iter() {
        queue.push_back(x);
    }

    let f = |x, y| (x + y) % 10;
    let g = |x, y| (x * y) % 10;

    // dp[i][j] := i 回目までの操作を行った時に左に追加される数が j になる場合の数数
    let mut dp = vec![vec![0usize; 10]; n + 1];
    dp[0][queue[0]] = 1;
    queue.pop_front();

    for i in 0..n - 1 {
        for j in 0..10 {
            let jf = f(j, queue[0]);
            let jg = g(j, queue[0]);
            dp[i + 1][jf] = (dp[i + 1][jf] + dp[i][j]) % MOD;
            dp[i + 1][jg] = (dp[i + 1][jg] + dp[i][j]) % MOD;
        }
        queue.pop_front();
    }

    for k in 0..10 {
        println!("{}", dp[n - 1][k]);
    }
}
