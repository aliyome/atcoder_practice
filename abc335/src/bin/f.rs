use proconio::input;
use std::collections::{HashSet, VecDeque};

fn main() {
    // 入力を受け取る
    input! {
        n: usize,
        mut a: [usize; n],
    }

    // 1-indexedにする
    a.insert(0, 0);

    // 各スクエアに対するエッジリストを用意
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n + 1];

    // グラフの構築
    for i in 1..=n {
        let mut x = 1;
        while i + a[i] * x <= n {
            graph[i].insert(i + a[i] * x);
            x += 1;
        }
    }

    // 動的計画法のテーブルを初期化
    let mut dp = vec![0; n + 1];
    dp[1] = 1; // スクエア1には最初からいる

    // 動的計画法を適用
    for i in 1..=n {
        for &j in &graph[i] {
            dp[j] = (dp[j] + dp[i]) % 998244353;
        }
    }

    // 結果の集計
    let mut result = 0;
    for i in 1..=n {
        result = (result + dp[i]) % 998244353;
    }

    // 結果を出力
    println!("{}", result);
}
