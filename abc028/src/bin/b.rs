use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut map = BTreeMap::new();
    map.insert('A', 0);
    map.insert('B', 0);
    map.insert('C', 0);
    map.insert('D', 0);
    map.insert('E', 0);
    map.insert('F', 0);
    for &c in &s {
        *map.entry(c).or_insert(0) += 1;
    }

    for (_, v) in map {
        print!("{} ", v);
    }
}
