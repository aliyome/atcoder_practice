use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn dfs(g: &HashMap<usize, Vec<usize>>, vis: &mut Vec<bool>, v: usize) {
    vis[v] = true;
    for &next_v in &g[&v] {
        if !vis[next_v] {
            dfs(g, vis, next_v);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut g = HashMap::new();
    for i in 0..n {
        g.insert(i, Vec::new());
    }
    for i in 0..n {
        for j in i + 1..n {
            let mut diff = 0;
            for k in 0..m {
                if s[i][k] != s[j][k] {
                    diff += 1;
                }
            }
            if diff == 1 {
                g.get_mut(&i).unwrap().push(j);
                g.get_mut(&j).unwrap().push(i);
            }
        }
    }
    let mut vis = vec![false; n];
    dfs(&g, &mut vis, 0);
    if vis.into_iter().all(|x| x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
