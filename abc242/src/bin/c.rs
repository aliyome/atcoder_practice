use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
    };

    let mut dp = vec![vec![0; 11]; n + 1];
    for i in 1..=9 {
        dp[1][i] = 1;
    }
    for i in 2..=n {
        for j in 1..=9 {
            dp[i][j] += dp[i - 1][j - 1];
            dp[i][j] += dp[i - 1][j];
            dp[i][j] += dp[i - 1][j + 1];
            dp[i][j] %= MOD
        }
    }

    let mut ans = 0;
    for i in 1..=9 {
        ans += dp[n][i];
    }

    println!("{}", ans);
}

fn f(left: usize, last: i64) -> usize {
    if left == 0 {
        return 1;
    }
    if last - 1 <= 0 {
        return f(left - 1, last) + f(left - 1, last + 1);
    }
    if last + 1 >= 0 {
        return f(left - 1, last - 1) + f(left - 1, last);
    }
    f(left - 1, last - 1) + f(left - 1, last) + f(left - 1, last + 1)
}
