use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, usize); n]
    }
    // t をキーとするハッシュにする
    let mut data: HashMap<usize, (usize, usize)> = HashMap::new();
    for i in 0..n {
        data.insert(txa[i].0, (txa[i].1, txa[i].2));
    }

    let mut dp = vec![vec![0; 5]; 10usize.pow(5) + 10];
    for t in (0..=10usize.pow(5)).rev() {
        let mut snuke_x = 0;
        let mut snuke_a = 0;
        if data.contains_key(&t) {
            snuke_x = data.get(&t).unwrap().0;
            snuke_a = data.get(&t).unwrap().1;
        }
        for x in 0..5 {
            if x == 0 {
                dp[t][x] = dp[t + 1][x].max(dp[t + 1][x + 1]);
            } else if x == 4 {
                dp[t][x] = dp[t + 1][x].max(dp[t + 1][x - 1]);
            } else {
                dp[t][x] = dp[t + 1][x].max(dp[t + 1][x - 1]).max(dp[t + 1][x + 1]);
            }
            if x == snuke_x {
                dp[t][x] += snuke_a;
            }
        }
    }
    println!("{}", dp[0][0]);
}
