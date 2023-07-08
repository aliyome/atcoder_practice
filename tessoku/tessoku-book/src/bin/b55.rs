use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
      q: usize,
      tx: [(usize, isize); q]
    }

    let mut set = BTreeSet::new();
    for i in 0..q {
        let (t, x) = tx[i];
        if t == 1 {
            set.insert(x);
        } else {
            let inf = 10isize.pow(12);
            let c1 = *set.range(x..).next().unwrap_or(&inf);
            let c2 = *set.range(..x).next_back().unwrap_or(&inf);
            if c1 == inf && c2 == inf {
                println!("-1");
            } else {
                println!("{}", (c1 - x).abs().min((c2 - x).abs()));
            }
        }
    }
}
