use std::collections::{HashMap, HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    };

    let mut graph = vec![vec![]; n + 1];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    // UnionFindで連結成分を調べる
    let mut uf = UnionFind::new(n + 1);
    for &(u, v) in &uv {
        uf.unite(u, v);
    }
    let mut roots = HashSet::new();
    for i in 1..=n {
        roots.insert(uf.find(i));
    }

    // 二部グラフの色塗り
    let mut color = vec![0; n + 1];
    let mut queue = VecDeque::new();
    for &root in &roots {
        queue.push_back((root, 1));
        color[root] = 1;
    }
    // 二部グラフ判定
    let mut is_not_bipartite = vec![];
    while let Some((v, c)) = queue.pop_front() {
        for &u in &graph[v] {
            if c == color[u] {
                is_not_bipartite.push(uf.find(u));
            }
            if color[u] != 0 {
                continue;
            }
            color[u] = if c == 1 { 2 } else { 1 };
            queue.push_back((u, color[u]));
        }
    }
    let mut bipartite_roots = HashSet::new();
    for &root in &roots {
        if !is_not_bipartite.contains(&root) {
            bipartite_roots.insert(root);
        }
    }

    // 各連結成分ごとの色の数を数える
    let mut color_count = HashMap::new();
    for i in 1..=n {
        let root = uf.find(i);
        let c = color[i];
        let x = color_count.entry(root).or_insert((0, 0));
        if c == 1 {
            x.0 += 1;
        } else if c == 2 {
            x.1 += 1;
        }
    }

    // 二部グラフ判定
    let mut ans = 0;
    for i in 1..=n {
        let root = uf.find(i);
        if is_not_bipartite.contains(&root) {
            continue;
        }

        // 同じ連結成分は別の色のみ
        let edge_count = graph[i].len();
        let (c1, c2) = color_count.get(&root).unwrap();
        if color[i] == 1 && *c2 > edge_count {
            ans += *c2 - edge_count;
        } else if color[i] == 2 && *c1 > edge_count {
            ans += *c1 - edge_count;
        }

        // 別の連結成分はどの頂点でもOK
        for &root in &bipartite_roots {
            let (c1, c2) = color_count.get(&root).unwrap();
            ans += c1 + c2;
        }
    }

    println!("{}", ans);
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    // O(N)
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    // O(α(N)) ≒ O(1)
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        }
    }

    // O(α(N)) ≒ O(1)
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.parent[y] = x;
            self.size[x] += self.size[y];
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    // O(α(N)) ≒ O(1)
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
