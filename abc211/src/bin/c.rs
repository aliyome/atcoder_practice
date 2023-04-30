use proconio::{input, marker::Chars};

const MOD: usize = 1000000007;

fn main() {
    input! {
        s: Chars,
    };

    let chokudai = "chokudai".chars().collect::<Vec<char>>();
    let input_len = s.len();
    let chokudai_len = chokudai.len();

    let mut dp = vec![vec![0; input_len + 1]; chokudai_len + 1];
    // 番兵
    dp[0] = vec![1; input_len + 1];

    for i in 0..chokudai_len {
        for j in 0..input_len {
            if chokudai[i] != s[j] {
                dp[i + 1][j + 1] = dp[i + 1][j];
            } else {
                dp[i + 1][j + 1] = (dp[i + 1][j] + dp[i][j]) % MOD;
            }
        }
    }

    println!("{}", dp[chokudai_len][input_len]);
}
