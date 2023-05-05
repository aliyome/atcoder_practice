use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        coins: [(i32, i32); n],
    }

    let mut dp = vec![vec![false; x as usize + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=x as usize {
            if dp[i][j] {
                dp[i + 1][j] = true;
                for k in 0..=coins[i].1 {
                    let new_value = j as i32 + coins[i].0 * k;
                    if new_value <= x {
                        dp[i + 1][new_value as usize] = true;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    if dp[n][x as usize] {
        println!("Yes");
    } else {
        println!("No");
    }
}
