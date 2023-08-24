use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    // 全コインを一つの配列にまとめる
    let mut coins = vec![];
    let mut all_coins = 0;
    for &(a, b) in &ab {
        for _ in 0..b {
            coins.push(a);
        }
        all_coins += b;
    }

    // dp[i][j] := i枚のコインを使ってj円の支払いを支払うことができるか
    let mut dp = vec![vec![false; x + 1]; all_coins + 1];
    dp[0][0] = true;

    for i in 0..all_coins {
        dp[i + 1] = dp[i].clone();
        for j in 0..=x {
            if !dp[i][j] {
                continue;
            }
            if j + coins[i] > x {
                continue;
            }
            dp[i + 1][j + coins[i]] = true;
        }
    }

    for i in 0..=all_coins {
        if dp[i][x] {
            println!("Yes");
            return;
        }
        // println!("{:?}", dp[i]);
    }

    println!("No");
}
