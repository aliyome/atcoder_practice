use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q]
    };

    let mut last_change = std::usize::MAX;
    let mut last_case = std::usize::MAX;
    let mut after_changed = HashSet::new();
    for i in 0..q {
        let (t, x, c) = txc[i];
        if t == 1 {
            s[x - 1] = c;
            after_changed.insert(x - 1);
        } else {
            last_change = i;
            last_case = t;
            after_changed.clear();
        }
    }

    if last_change == std::usize::MAX {
        for i in 0..n {
            print!("{}", s[i]);
        }
        return;
    }

    for i in 0..n {
        if last_case == 2 {
            if after_changed.contains(&i) {
                print!("{}", s[i]);
            } else {
                print!("{}", s[i].to_lowercase());
            }
        } else {
            if after_changed.contains(&i) {
                print!("{}", s[i]);
            } else {
                print!("{}", s[i].to_uppercase());
            }
        }
    }
}
