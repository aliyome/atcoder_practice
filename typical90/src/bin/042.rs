use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        k: usize
    }

    // (N mod 9) = (N の各桁の和 mod 9)
    if k % 9 != 0 {
        println!("0");
        return;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 1..=k {
        let b = i.min(9);
        for j in 1..=b {
            dp[i] += dp[i - j];
            dp[i] %= MOD;
        }
    }

    println!("{:?}", dp[k]);
}
