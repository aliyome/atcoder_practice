use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        k: usize
    }

    // (N mod 9) = (N の各桁の和 mod 9)
    // 各桁の数字の和が9の倍数で無いなら、9の倍数xはkになれない
    if k % 9 != 0 {
        println!("0");
        return;
    }

    // dp[i] := 各桁の数字の和がiとなる数の個数
    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=k {
        for j in 1..=9 {
            if i >= j {
                dp[i] += dp[i - j];
                dp[i] %= MOD;
            }
        }
    }

    println!("{}", dp[k]);
}
