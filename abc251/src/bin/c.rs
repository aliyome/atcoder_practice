use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    };

    let mut original = HashSet::new();
    let mut max = 0;
    let mut max_i = 0;
    for i in 0..n {
        let (s, t) = &st[i];
        if !original.contains(s) {
            if max < *t {
                max = *t;
                max_i = i;
            }
        }
        original.insert(s);
    }

    println!("{}", max_i + 1);
}
