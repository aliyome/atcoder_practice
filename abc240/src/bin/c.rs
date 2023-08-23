use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    };

    let mut dp = vec![vec![false; 20000]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..20000 {
            if !dp[i][j] {
                continue;
            }
            dp[i + 1][j + a] = true;
            dp[i + 1][j + b] = true;
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
