use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = 0;
    let mut set = HashSet::new();
    for s in s.iter() {
        if set.contains(s) {
            continue;
        }
        set.insert(s.clone());
        set.insert(s.chars().rev().collect::<String>());
        ans += 1;
    }

    println!("{}", ans);
}
