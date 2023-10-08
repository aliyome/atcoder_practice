use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    }

    let mut map = BTreeMap::new();
    for &(s, c) in &sc {
        map.insert(s, c);
    }

    let mut count = 0;
    while let Some((s, c)) = map.pop_first() {
        let rest = c % 2;
        let combined = c / 2;
        count += rest;
        if combined > 0 {
            *map.entry(s * 2).or_insert(0) += combined;
        }
    }

    println!("{}", count);
}
