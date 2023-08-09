use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut map = HashMap::new();
    for i in 0..n {
        if map.contains_key(&s[i]) {
            println!("{}({})", s[i], map[&s[i]]);
        } else {
            println!("{}", s[i]);
        }
        *map.entry(&s[i]).or_insert(0) += 1usize;
    }
}
