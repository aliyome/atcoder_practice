use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q]
    };

    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(l);

    for &(c, x) in &cx {
        if c == 1 {
            set.insert(x);
        } else {
            let min = set.range(..x).next_back().unwrap();
            let max = set.range(x..).next().unwrap();
            println!("{}", max - min);
        }
    }
}
