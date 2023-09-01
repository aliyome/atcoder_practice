use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for j in 1..=m {
        dp[1][j] = 1;
    }

    // iまでみて
    for i in 1..n {
        // 最後がjの時
        for j in 1..=m {
            // 次がlだとして
            for l in 1..=m {
                // 条件を満たす場合
                if (j as isize - l as isize).abs() >= k as isize {
                    // それまでの場合の数を足す
                    dp[i + 1][l] += dp[i][j];
                    dp[i + 1][l] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[n].iter().sum::<usize>());
}
