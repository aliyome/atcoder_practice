use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n]
    };

    // 条件
    let is_ok = |x0: isize, x1: isize| (x0 - x1).abs() <= k;

    // dp[i][j] = i 番目にj(0:a, 1:b)を選んだときに条件を満たしているか
    let mut dp = vec![vec![false; 2]; n + 1];
    dp[0][0] = true;
    dp[0][1] = true;

    for i in 1..n {
        dp[i][0] =
            (dp[i - 1][0] && is_ok(a[i - 1], a[i])) || (dp[i - 1][1] && is_ok(b[i - 1], a[i]));
        dp[i][1] =
            (dp[i - 1][0] && is_ok(a[i - 1], b[i])) || (dp[i - 1][1] && is_ok(b[i - 1], b[i]));
    }

    if dp[n - 1][0] || dp[n - 1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
