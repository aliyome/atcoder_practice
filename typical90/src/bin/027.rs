use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        if !set.contains(&s[i]) {
            println!("{}", i + 1);
        }
        set.insert(s[i].clone());
    }
}
