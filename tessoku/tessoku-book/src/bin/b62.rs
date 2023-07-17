use proconio::input;

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

    let mut ans = vec![];
    dfs(&graph, 1, &mut vec![false; n + 1], &mut vec![], &mut ans);

    for &a in &ans {
        print!("{} ", a);
    }
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    v: usize,
    visited: &mut Vec<bool>,
    path: &mut Vec<usize>,
    ans: &mut Vec<usize>,
) {
    visited[v] = true;
    path.push(v);

    if v == graph.len() - 1 {
        ans.clear();
        ans.extend(path.clone());
        return;
    }

    for &u in &graph[v] {
        if visited[u] {
            continue;
        }
        dfs(graph, u, visited, path, ans);
        path.pop();
    }
}
