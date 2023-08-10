use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    // dp[i][j] := i文字目までみて、最後がj文字の場合の分割数
    let inf = std::isize::MIN;
    let mut dp = vec![vec![inf; 4]; s.len() + 2];
    dp[0][0] = 0;

    // i
    for i in 0..s.len() {
        for j in 0..4 {
            for k in 1..4 {
                if i + k > s.len() {
                    continue;
                }
                if i < j {
                    continue;
                }
                // 同じ文字列の場合はスキップ
                let a = s[i - j..i].iter().collect_vec();
                let b = s[i..i + k].iter().collect_vec();
                if a == b {
                    continue;
                }
                dp[i + k][k] = dp[i + k][k].max(dp[i][j] + 1);
            }
        }
    }

    println!("{}", dp[s.len()].iter().max().unwrap());
}
