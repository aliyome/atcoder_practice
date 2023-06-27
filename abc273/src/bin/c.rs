use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut sorted = a.clone();
    sorted.sort();

    let mut dedup = sorted.clone();
    dedup.dedup();

    let uniq_keys = dedup.len();
    let mut num_of_larger = HashMap::new();
    let mut k_map = HashMap::new();
    for (i, &x) in dedup.iter().enumerate() {
        let count = uniq_keys - i - 1;
        num_of_larger.insert(x, count);
        k_map.insert(count, x);
    }

    let mut counts = HashMap::new();
    for &a in &a {
        *counts.entry(a).or_insert(0) += 1;
    }

    // println!("{:?}", sorted);
    // println!("{:?}", dedup);
    // println!("{:?}", num_of_larger);
    // println!("{:?}", k_map);
    // println!("{:?}", counts);

    for k in 0..n {
        let a = k_map.get(&k).unwrap_or(&0);
        let count = counts.get(a).unwrap_or(&0);
        println!("{}", *count);
    }
}
