use proconio::input;

// v 初めて訪問した頂点
fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, v: usize) {
    visited[v] = true;
    for &u in &graph[v] {
        if !visited[u] {
            dfs(visited, graph, u);
        }
    }
}

fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m]
    }

    let mut graph = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n + 1];
    visited[0] = true;
    dfs(&mut visited, &graph, 1);
    if visited.iter().all(|&v| v) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }

    // let mut stack = vec![];
    // let mut visited = vec![false; n + 1];
    // visited[0] = true;
    // visited[1] = true;

    // stack.push(1);

    // while let Some(v) = stack.pop() {
    //     for &u in &graph[v] {
    //         if visited[u] {
    //             continue;
    //         }
    //         visited[u] = true;
    //         stack.push(u);
    //     }
    // }

    // if visited.iter().all(|&v| v) {
    //     println!("The graph is connected.");
    // } else {
    //     println!("The graph is not connected.");
    // }

    // let mut uf = UnionFind::new(n);
    // for &(a, b) in &ab {
    //     uf.unite(a - 1, b - 1);
    // }

    // let root = uf.find(0);
    // for i in 0..n {
    //     if uf.find(i) != root {
    //         println!("The graph is not connected.");
    //         return;
    //     }
    // }

    // println!("The graph is connected.");
}

// pub struct UnionFind {
//     parent: Vec<usize>,
//     rank: Vec<usize>,
//     size: Vec<usize>,
// }

// impl UnionFind {
//     // O(N)
//     fn new(n: usize) -> Self {
//         UnionFind {
//             parent: (0..n).collect(),
//             rank: vec![0; n],
//             size: vec![1; n],
//         }
//     }

//     // O(α(N)) ≒ O(1)
//     fn find(&mut self, x: usize) -> usize {
//         if self.parent[x] == x {
//             x
//         } else {
//             let p = self.find(self.parent[x]);
//             self.parent[x] = p;
//             p
//         }
//     }

//     // O(α(N)) ≒ O(1)
//     fn unite(&mut self, x: usize, y: usize) {
//         let x = self.find(x);
//         let y = self.find(y);
//         if x == y {
//             return;
//         }
//         if self.rank[x] < self.rank[y] {
//             self.parent[x] = y;
//             self.size[y] += self.size[x];
//         } else {
//             self.parent[y] = x;
//             self.size[x] += self.size[y];
//             if self.rank[x] == self.rank[y] {
//                 self.rank[x] += 1;
//             }
//         }
//     }

//     // O(α(N)) ≒ O(1)
//     fn same(&mut self, x: usize, y: usize) -> bool {
//         self.find(x) == self.find(y)
//     }
// }
