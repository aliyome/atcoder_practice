use proconio::input;

const MOD: i64 = 1_000_000_007;

// TODO: 解説見てもよくわからんな…
// https://www.youtube.com/watch?v=HVuSp_IhNZA
fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut ans: i64 = 0;
    for i in k..=n + 1 {
        let min_sum = i * (i - 1) / 2;
        let max_sum = i * (2 * n - i + 1) / 2;
        ans = (ans + (max_sum - min_sum + 1) % MOD) % MOD;
    }

    println!("{}", ans);
}
