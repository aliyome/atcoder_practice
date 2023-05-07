use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }

    // 重み付き有向グラフの最短経路はダイクストラ法で求められる

    // 重み付き有向グラフを構築する
    // graph[from] = [(to, cost), ...]
    let mut graph = vec![vec![]; n + 1];
    for (a, b, c) in abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    // ダイクストラ法で両端から最短経路を求める
    let dist_from_1_to = dijkstra(&graph, 1);
    let dist_from_n_to = dijkstra(&graph, n);

    for k in 1..=n {
        // cost(1 -> k) + cost(k -> n)
        println!("{}", dist_from_1_to[k] + dist_from_n_to[k]);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, from: usize) -> Vec<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist = vec![std::usize::MAX; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[from] = 0;
    heap.push(Reverse((0, from)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if dist[node] < cost {
            continue;
        }

        for &(next_node, next_cost) in &graph[node] {
            let next_cost = cost + next_cost;
            if next_cost < dist[next_node] {
                dist[next_node] = next_cost;
                heap.push(Reverse((next_cost, next_node)));
            }
        }
    }

    dist
}
