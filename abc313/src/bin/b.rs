use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut dp = vec![vec![false; n]; n];
    for (a, b) in ab {
        dp[a - 1][b - 1] = true;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dp[i][k] && dp[k][j] {
                    dp[i][j] = true;
                }
            }
        }
    }
    let mut strongest = None;
    for i in 0..n {
        if (0..n).all(|j| i == j || dp[i][j]) {
            if strongest.replace(i).is_some() {
                println!("-1");
                return;
            }
        }
    }
    if let Some(i) = strongest {
        println!("{}", i + 1);
    } else {
        println!("-1");
    }
}
