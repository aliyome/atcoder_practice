use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    // dp[k][i][j] :=
    // ai まで見て、
    // 最後の状態が j (0:a[i-1]を使わない、1:a[i-1]を使う) の時、
    // 最初が k (0:a[0]を使わない、1:a[0]を使う) であるような場合の最小値
    let inf = std::usize::MAX;
    let mut dp = vec![vec![vec![inf; 2]; n + 1]; 2];

    // 最初が k で
    for k in 0..2 {
        if k == 0 {
            dp[k][1][0] = 0;
            // dp[k][1][1] = 0;
        } else {
            dp[k][1][1] = a[0];
        }

        // ai まで見て
        for i in 1..n {
            // 最後の状態が j のとき
            for j in 0..2 {
                if dp[k][i][j] == inf {
                    continue;
                }
                if j == 1 {
                    // 最後の状態が j:1 ならば次の状態は 0,1 どちらでもよい
                    dp[k][i + 1][1] = dp[k][i + 1][1].min(dp[k][i][j] + a[i]);
                    dp[k][i + 1][0] = dp[k][i + 1][0].min(dp[k][i][j]);
                } else {
                    // 最後の状態が j:0 ならば次の状態は必ず 1 になる。連続で 0 はダメ
                    dp[k][i + 1][1] = dp[k][i + 1][1].min(dp[k][i][j] + a[i]);
                }
            }
        }
    }

    // 最初が 0 なら最後は必ず 1 になる。最初が 1 なら最後は 0,1 どちらでもよい
    let ans = dp[0][n][1].min(dp[1][n][0]).min(dp[1][n][1]);
    println!("{}", ans);
}
