use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    };

    let mut map = HashMap::new();

    for i in 0..n {
        map.entry(a[i])
            .or_insert(BTreeSet::new())
            .insert((i + 1, i + 1));
    }

    for &(l, r, x) in &lrx {
        if !map.contains_key(&x) {
            println!("0");
            continue;
        }
        let list = map.get(&x).unwrap();
        let ans = list.range((l, 0)..=(r, n + 1)).count();
        println!("{}", ans);
    }
}
