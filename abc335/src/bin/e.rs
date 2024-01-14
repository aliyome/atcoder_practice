use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, // 頂点の数
        m: usize, // 辺の数
        a: [usize; n], // 各頂点の値
        edges: [(Usize1, Usize1); m], // 各辺の情報
    }

    let mut graph = vec![vec![]; n];
    let mut indegree = vec![0; n];
    for &(u, v) in &edges {
        if a[u] <= a[v] {
            graph[u].push(v);
            indegree[v] += 1;
        }
        if a[u] >= a[v] {
            graph[v].push(u);
            indegree[u] += 1;
        }
    }

    let mut queue = VecDeque::new();
    let mut lis = vec![0; n]; // 各頂点までの最長の非減少パスの長さの初期値を0に設定
    lis[0] = 1; // 始点だけはパス長さ1とする
    queue.push_back(0); // 始点をキューに追加
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if a[u] <= a[v] && lis[u] + 1 > lis[v] {
                lis[v] = lis[u] + 1; // 非減少条件を満たす場合のみ更新
            }
            indegree[v] -= 1;
            if indegree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    // 終点までの非減少パスが存在しない場合は0を出力
    println!("{}", if lis[n - 1] == 0 { 0 } else { lis[n - 1] });
}
