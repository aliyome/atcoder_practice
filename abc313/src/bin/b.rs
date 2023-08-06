use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        edges[b].push(a);
    }

    let mut strongest = HashSet::new();
    for i in 1..=n {
        let mut stack = vec![];
        let mut visited = vec![false; n + 1];
        stack.push(i);
        visited[i] = true;
        while let Some(v) = stack.pop() {
            if edges[v].is_empty() {
                strongest.insert(v);
            } else {
                for &u in &edges[v] {
                    if !visited[u] {
                        visited[u] = true;
                        stack.push(u);
                    }
                }
            }
        }
    }

    if strongest.len() == 1 {
        println!("{}", strongest.iter().next().unwrap());
    } else {
        println!("-1");
    }
}
