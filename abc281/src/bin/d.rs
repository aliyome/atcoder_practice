use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [isize; n]
    };

    // dp[i][j][x] := i番目まで見てj個選んだ時にdで割ったあまりがxであるような最大値
    let mut dp = vec![vec![vec![std::isize::MIN; d]; k + 2]; n + 1];
    dp[0][0][0] = 0;

    // i まで見て
    for i in 0..n {
        let a = a[i];
        // すでに j 個選んだ時に
        for j in 0..=k {
            // d で割ったあまりが x であるような
            for x in 0..d {
                // a[i] を選ばない場合
                dp[i + 1][j][x] = dp[i + 1][j][x].max(dp[i][j][x]);
                // a[i] を選ぶ場合
                let sum = dp[i][j][x] + a;
                let y = sum as usize % d;
                dp[i + 1][j + 1][y] = dp[i + 1][j + 1][y].max(sum);
            }
        }
    }

    if dp[n][k][0] < 0 {
        println!("-1");
        return;
    }

    println!("{}", dp[n][k][0]);
}
