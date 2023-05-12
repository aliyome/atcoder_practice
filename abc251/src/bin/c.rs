use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    };

    let mut map = HashMap::new();
    for i in 0..n {
        let (s, t) = &st[i];
        if !map.contains_key(s) {
            map.insert(s, (*t, i + 1));
        }
    }

    let mut list = map.iter().collect::<Vec<_>>();
    list.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.1 .1.cmp(&b.1 .1)));

    println!("{}", list[0].1 .1);
}
