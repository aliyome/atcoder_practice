use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut edge = vec![BTreeSet::new(); n + 1];
    for i in 0..m {
        let (a, b) = ab[i];
        edge[a].insert((b, i));
        edge[b].insert((a, i));
    }

    for i in 1..=n {
        print!("{} ", edge[i].len());
        for j in edge[i].iter() {
            print!("{} ", j.0);
        }
        println!();
    }
}
