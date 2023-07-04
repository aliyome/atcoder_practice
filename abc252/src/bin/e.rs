use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    };

    // ダイクストラ法で最短距離を求める
    let mut graph = vec![vec![]; n];
    let mut edges = HashMap::new();
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        graph[a - 1].push((b - 1, c));
        graph[b - 1].push((a - 1, c));

        edges.insert((a - 1, b - 1), i);
        edges.insert((b - 1, a - 1), i);
    }

    let path = dijkstra(&graph, 0);
    for (k, v) in &path {
        print!("{} ", edges.get(&(*k, *v)).unwrap() + 1);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, from: usize) -> HashMap<usize, usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist = vec![std::usize::MAX; graph.len()];
    let mut heap = BinaryHeap::new();
    let mut path = HashMap::new();

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
                path.insert(next_node, node);
                heap.push(Reverse((next_cost, next_node)));
            }
        }
    }

    path
}
