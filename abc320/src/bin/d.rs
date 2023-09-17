use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(usize, usize, isize, isize); m],
    }

    let mut edge = vec![vec![]; n + 1];
    for &(a, b, x, y) in &abxy {
        edge[a].push((b, x, y));
        edge[b].push((a, -x, -y));
    }

    let inf = std::isize::MAX;
    let mut pos = vec![(inf, inf); n + 1];
    pos[1] = (0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(1);
    while let Some(v) = queue.pop_front() {
        for &(u, x, y) in &edge[v] {
            if pos[u] == (inf, inf) {
                pos[u] = (pos[v].0 + x, pos[v].1 + y);
                queue.push_back(u);
            }
        }
    }

    for i in 1..=n {
        if pos[i] == (inf, inf) {
            println!("undecidable");
        } else {
            println!("{} {}", pos[i].0, pos[i].1);
        }
    }
}
