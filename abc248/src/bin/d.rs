use std::collections::HashMap;
use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    };

    // 1-indexed
    a.insert(0, 0);

    // map[c] := c が出現する位置の配列
    let mut map = HashMap::new();
    for i in 1..=n {
        let c = a[i];
        map.entry(c).or_insert(vec![]).push(i);
    }

    for &(l, r, x) in &lrx {
        if !map.contains_key(&x) {
            println!("0");
            continue;
        }
        let list = map.get(&x).unwrap();
        let lc = list.lower_bound(&l);
        let rc = list.upper_bound(&r);
        println!("{}", rc - lc);
    }
}
