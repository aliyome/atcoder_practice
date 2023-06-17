use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 3]
    };

    let mut map = HashMap::new();

    for i in 0..n * 3 {
        map.entry(a[i]).or_insert(vec![]).push(i);
    }

    let mut list = vec![];
    for (k, v) in map {
        list.push((k, v[1]));
    }

    list.sort_by(|a, b| a.1.cmp(&b.1));

    for (k, _) in list {
        print!("{} ", k);
    }
}
