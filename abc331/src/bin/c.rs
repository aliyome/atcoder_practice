use std::collections::{BTreeMap, BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut aa = a.clone();
    aa.sort();

    let mut sum = aa.iter().sum::<usize>();

    let mut map = BTreeMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut ans_map = HashMap::new();
    for (k, v) in map {
        ans_map.insert(k, sum - v * k);
        sum -= v * k;
    }

    for &a in &a {
        print!("{} ", ans_map.get(&a).unwrap());
    }
}
