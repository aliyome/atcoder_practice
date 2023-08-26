use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }

    let mut edge = HashMap::new();
    for &(a, b, c) in &abc {
        edge.entry(a).or_insert(vec![]).push((b, c));
        edge.entry(b).or_insert(vec![]).push((a, c));
    }

    let mut dist = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        let mut visited = vec![false; n + 1];
        visited[0] = true;
        visited[i] = true;
        dfs(i, i, 0, &edge, &mut dist, &mut visited);
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            ans = ans.max(dist[i][j]);
        }
    }

    println!("{}", ans);
}

fn dfs(
    start: usize,
    i: usize,
    sum: usize,
    edge: &HashMap<usize, Vec<(usize, usize)>>,
    dist: &mut Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
) {
    dist[start][i] = dist[start][i].max(sum);

    if let Some(list) = edge.get(&i) {
        for &(j, c) in list {
            if visited[j] {
                continue;
            }
            visited[j] = true;
            dfs(start, j, sum + c, edge, dist, visited);
            visited[j] = false;
        }
    }
}
