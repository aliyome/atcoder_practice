use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = HashSet::new();
    for s in s {
        let rev = s.chars().rev().collect::<String>();
        if !set.contains(&s) && !set.contains(&rev) {
            set.insert(s);
        }
    }

    println!("{}", set.len());
}
