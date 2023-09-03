use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut indices = BTreeMap::new();
    for i in 0..n {
        indices.entry(a[i]).or_insert(vec![]).push(i);
    }

    // println!("{:?}", indices);

    let mut ans = 0;
    for (k, v) in &indices {
        if v.len() < 2 {
            continue;
        }
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let x = v[j] - v[i] - 1;
                let y = v[i + 1..j].len();
                // let x = v.range(j..).next().unwrap() - v.range(i).unwrap() - 1;
                // let y = v.range(i + 1..j).count();
                ans += x - y;
                // println!("{} {} {} {} {}", k, i, j, x, y);
            }
        }
    }

    println!("{}", ans);
}
