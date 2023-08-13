use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut map = BTreeMap::new();
    let mut max = 0;
    for s in s {
        *map.entry(s.clone()).or_insert(0) += 1usize;
        max = max.max(map[&s]);
    }

    for (k, v) in map {
        if v == max {
            println!("{}", k);
        }
    }
}
