use proconio::input;
use std::collections::VecDeque;

fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> (usize, usize) {
    let mut stack = VecDeque::new();
    stack.push_back(v);
    visited[v] = true;
    let mut vertices = 1;
    let mut edges = 0;

    while let Some(current) = stack.pop_back() {
        for &next in &graph[current] {
            edges += 1;
            if !visited[next] {
                visited[next] = true;
                vertices += 1;
                stack.push_back(next);
            }
        }
    }
    (vertices, edges)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut graph = vec![Vec::new(); n + 1];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n + 1];
    let mut result = "Yes";

    for i in 1..=n {
        if !visited[i] {
            let (vertices, edges) = dfs(i, &graph, &mut visited);
            if 2 * vertices != edges {
                result = "No";
                break;
            }
        }
    }

    println!("{}", result);
}
