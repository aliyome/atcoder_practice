use std::collections::HashMap;

use proconio::input;

// 026 - Independent Set on a Tree（★4）
// https://atcoder.jp/contests/typical90/tasks/typical90_z
// 解説が意味不明だったけど何も考えずにDFSで良かった

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }

    // 無向グラフ
    let mut graph = HashMap::new();
    for (a, b) in ab.iter() {
        graph.entry(*a).or_insert(vec![]).push(*b);
        graph.entry(*b).or_insert(vec![]).push(*a);
    }

    // スタックを使った DFS
    let mut stack = vec![];
    let mut visited = vec![false; n + 1];
    let mut groups = vec![0; n + 1];
    let mut group_count = [0, 0];

    stack.push((1, 1)); // (node, group)
    while let Some((node, group)) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        groups[node] = group;
        group_count[group] += 1;
        let next_group = if group == 1 { 0 } else { 1 };
        for &next in graph.get(&node).unwrap() {
            stack.push((next, next_group));
        }
    }
    let major = if group_count[0] < group_count[1] {
        1
    } else {
        0
    };

    let mut count = 0;
    let mut ans = vec![];
    for i in 1..=n {
        if groups[i] == major {
            ans.push(i);
            count += 1;
            if count == n / 2 {
                break;
            }
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
