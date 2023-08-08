use std::collections::HashMap;
use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    };

    let mut map = HashMap::new();

    for i in 0..n {
        map.entry(a[i]).or_insert(vec![]).push(i + 1);
    }

    for &(l, r, x) in &lrx {
        if !map.contains_key(&x) {
            println!("0");
            continue;
        }
        let list = map.get(&x).unwrap();
        let i = list.lower_bound(&l);
        let j = list.upper_bound(&r);
        println!("{}", j - i);
    }
}
