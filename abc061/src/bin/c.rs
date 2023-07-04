use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    };

    // let mut arr = vec![];
    // for i in 0..n {
    //     let (a, b) = ab[i];
    //     for j in 0..b {
    //         arr.push(a);
    //     }
    // }
    // println!("{}", arr[k - 1]);

    let mut map = BTreeMap::new();
    for i in 0..n {
        let (a, b) = ab[i];
        *map.entry(a).or_insert(0) += b;
    }

    let mut cnt = 0;
    for (&key, &val) in map.iter() {
        cnt += val;
        if k <= cnt {
            println!("{}", key);
            return;
        }
    }
}
