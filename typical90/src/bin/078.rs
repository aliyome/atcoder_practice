use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut edges = vec![BTreeSet::new(); n + 1];
    for &(a, b) in &ab {
        edges[a].insert(b);
        edges[b].insert(a);
    }

    let mut ans = 0;
    for i in 1..=n {
        if edges[i].range(..i).count() == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
