use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _w: usize, _h: usize,
        n: usize,
        mut pq: [(usize, usize); n],
        a: usize,
        xa: [usize; a],
        b: usize,
        yb: [usize; b],
    }

    // O(N)
    let mut pieces = HashMap::new();
    for &(p, q) in &pq {
        // 二分探索 O(logA + logB)
        let x = xa.binary_search(&p).unwrap_err();
        let y = yb.binary_search(&q).unwrap_err();
        // println!("p: {}, q: {}, x: {}, y: {}", p, q, x, y);
        *pieces.entry((x, y)).or_insert(0usize) += 1;
    }

    let max = *pieces.values().max().unwrap();
    let mut min = *pieces.values().min().unwrap();
    if (a + 1) * (b + 1) > pieces.keys().len() {
        min = 0;
    }
    println!("{} {}", min, max);
}
