// 最大フロー問題 Max flow
use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abc: [(usize, usize, usize); m]
    }

    let mut flow = MaximumFlow::new(n + 1);
    for &(a, b, c) in &abc {
        flow.add_edge(a, b, c);
    }
    println!("{}", flow.max_flow(1, n));
}

#[derive(Debug, Clone, Default)]
struct Edge {
    next: usize,
    capacity: usize,
    prev: usize,
}

struct MaximumFlow {
    size: usize,
    used: Vec<bool>,
    graph: Vec<Vec<Edge>>,
}

impl MaximumFlow {
    fn new(size: usize) -> Self {
        Self {
            size,
            used: vec![false; size],
            graph: vec![vec![]; size],
        }
    }

    // 残余グラフの辺を追加
    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let from_size = self.graph[from].len();
        let to_size = self.graph[to].len();
        self.graph[from].push(Edge {
            next: to,
            capacity: cap,
            prev: to_size,
        });
        self.graph[to].push(Edge {
            next: from,
            capacity: 0,
            prev: from_size,
        });
    }

    // 深さ優先探索
    fn dfs(&mut self, pos: usize, goal: usize, f: usize) -> usize {
        if pos == goal {
            return f;
        }
        self.used[pos] = true;

        for i in 0..self.graph[pos].len() {
            // 容量 0 の辺は使えない
            if self.graph[pos][i].capacity == 0 {
                continue;
            }

            // すでに訪問した頂点にはもう一度行かない
            if self.used[self.graph[pos][i].next] {
                continue;
            }

            // 目的地までのパスを探す
            let flow = self.dfs(
                self.graph[pos][i].next,
                goal,
                self.graph[pos][i].capacity.min(f),
            );

            // フローを流せる場合、残余グラフの容量を flow だけ増減させる
            if flow >= 1 {
                self.graph[pos][i].capacity -= flow;
                let next = self.graph[pos][i].next;
                let prev = self.graph[pos][i].prev;
                self.graph[next][prev].capacity += flow;
                return flow;
            }
        }
        // すべての辺を探索しても見つからなかった
        0
    }

    // 頂点 s から頂点 t までの最大フローの総流量
    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.size];
            let f = self.dfs(s, t, std::usize::MAX);

            // フローを流せなくなったら操作終了
            if f == 0 {
                break;
            }
            flow += f;
        }
        flow
    }
}
