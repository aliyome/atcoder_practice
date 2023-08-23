use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        s: usize,
    };

    // dp[i] := 各桁に 3 以上の数字をつかって総和が i になるような数列の個数
    let mut dp = vec![0; 2001];
    dp[0] = 0;

    for i in 0..=s {
        if i >= 3 {
            dp[i] = 1;
        }
        for j in 3..=s {
            if i < j {
                break;
            }
            dp[i] += dp[i - j];
            dp[i] %= MOD;
        }
    }

    println!("{}", dp[s]);
}
