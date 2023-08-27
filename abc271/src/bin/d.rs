use std::process::exit;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n]
    };

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=s {
            if !dp[i][j] {
                continue;
            }
            if j + a <= s {
                dp[i + 1][j + a] = true;
            }
            if j + b <= s {
                dp[i + 1][j + b] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("No");
        return;
    }

    println!("Yes");
    dfs(n, s, &ab, &dp, &mut vec![]);
}

fn dfs(v: usize, sum: usize, ab: &Vec<(usize, usize)>, dp: &Vec<Vec<bool>>, path: &mut Vec<char>) {
    if v == 0 {
        path.reverse();
        println!("{}", path.iter().collect::<String>());
        exit(0);
    }

    let (a, b) = ab[v - 1];
    if sum >= a && dp[v - 1][sum - a] {
        path.push('H');
        dfs(v - 1, sum - a, ab, dp, path);
        path.pop();
    }
    if sum >= b && dp[v - 1][sum - b] {
        path.push('T');
        dfs(v - 1, sum - b, ab, dp, path);
        path.pop();
    }
}
