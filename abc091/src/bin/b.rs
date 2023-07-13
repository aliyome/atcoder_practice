use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    };

    let mut map = HashMap::new();
    for s in s.into_iter() {
        *map.entry(s).or_insert(0) += 1;
    }
    for t in t.into_iter() {
        *map.entry(t).or_insert(0) -= 1;
    }

    let ans = *map.values().max().unwrap_or(&0);
    println!("{}", ans.max(0));
}
