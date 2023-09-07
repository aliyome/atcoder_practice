use std::collections::{BTreeMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: u128
    }
    let mut divs = HashSet::new();
    let mut pairs = BTreeMap::new();
    let mut i = 1;
    while i * i <= k {
        if k % i == 0 {
            pairs.insert(i, k / i);
            divs.insert(i);
            divs.insert(k / i);
        }
        i += 1;
    }

    let mut triple = HashSet::new();
    for &a in &divs {
        for &b in &divs {
            if a == 0 || b == 0 {
                continue;
            }
            let c = k / (a * b);
            if c == 0 {
                continue;
            }
            if a * b * c != k {
                continue;
            }
            let t: (u128, u128, u128) = vec![a, b, c].into_iter().sorted().collect_tuple().unwrap();
            triple.insert(t);
        }
    }

    println!("{}", triple.len());
}
