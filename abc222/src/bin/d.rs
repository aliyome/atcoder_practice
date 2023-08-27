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

    // 数列のi番目の要素を見るとき
    for i in 0..n {
        // 累積和
        let mut acc = vec![0; 3001];
        for j in 0..=3000 {
            acc[j] = if j < 1 { 0 } else { acc[j - 1] } + dp[i][j];
            acc[j] %= MOD;
        }
        for j in a[i]..=b[i] {
            dp[i + 1][j] = acc[j];
        }
    }

    let mut ans = 0;
    for j in 0..=3000 {
        ans += dp[n][j];
        ans %= MOD;
    }

    println!("{}", ans);
}
