use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        // a は非負の数
        mut a: [isize; n]
    };
    // 1-indexed
    a.insert(0, 0);

    // dp[x][y][z] := x番目までの数をy個選んで、その和をdで割った余りがzであるような選び方をしたとき、その和の最大値
    let mut dp = vec![vec![vec![-1isize; d]; k + 2]; n + 1];
    dp[0][0][0] = 0;

    // 配るDP
    for x in 0..n {
        for y in 0..=k {
            for z in 0..d {
                if dp[x][y][z] == -1 {
                    // 配り元が無い場合
                    continue;
                }
                // a[x + 1] を選ばないとき
                dp[x + 1][y][z] = dp[x + 1][y][z].max(dp[x][y][z]);
                // a[x + 1] を選ぶとき
                let z_new = (z + a[x + 1] as usize) % d;
                dp[x + 1][y + 1][z_new] = dp[x + 1][y + 1][z_new].max(dp[x][y][z] + a[x + 1]);
            }
        }
    }

    let ans = dp[n][k][0];

    println!("{}", ans);
}
