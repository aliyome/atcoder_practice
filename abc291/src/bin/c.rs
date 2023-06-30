use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut p = (0isize, 0isize);
    let mut set = HashSet::new();
    set.insert(p);
    for &c in &s {
        match c {
            'L' => p.0 -= 1,
            'R' => p.0 += 1,
            'U' => p.1 += 1,
            'D' => p.1 -= 1,
            _ => unreachable!(),
        }
        if set.contains(&p) {
            println!("Yes");
            return;
        }
        set.insert(p);
    }

    println!("No");
}
