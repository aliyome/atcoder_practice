use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut b: [usize; n - 1]
    };

    b.insert(0, 0);
    b.insert(0, 0);

    // 子から親への辺を作る
    let mut edge_to_parent = vec![vec![]; n + 1];
    let mut edge_to_child = vec![vec![]; n + 1];
    for i in 1..=n {
        edge_to_parent[i].push(b[i]);
        edge_to_child[b[i]].push(i);
    }

    // 葉をキューに入れる
    let mut queue = VecDeque::new();
    for i in 0..=n {
        if edge_to_child[i].len() == 0 {
            queue.push_back(i);
        }
    }
    // println!("{:?}", queue);

    // 木DP(葉から枝へ配る)
    let mut dp = vec![0usize; n + 1];
    while let Some(i) = queue.pop_front() {
        // if dp[i] != 0 {
        //     continue;
        // }
        // 部下の最大値と最小値を求める
        let mut max = 0;
        let mut min = if edge_to_child[i].len() == 0 {
            0
        } else {
            std::usize::MAX
        };
        for &c in &edge_to_child[i] {
            max = max.max(dp[c]);
            min = min.min(dp[c]);
        }
        dp[i] = max + min + 1;

        for &p in &edge_to_parent[i] {
            queue.push_back(p);
        }
    }

    // println!("{:?}", edge_to_child);
    // println!("{:?}", dp);

    println!("{}", dp[1]);
}
