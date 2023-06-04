use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _w: usize, _h: usize,
        n: usize,
        pq: [(usize, usize); n],
        a: usize,
        xa: [usize; a],
        b: usize,
        yb: [usize; b],
    }

    // O(N)
    let total_piece = (a + 1) * (b + 1);
    let mut berry_map = HashMap::new();
    for (p, q) in pq {
        // 二分探索 O(logA + logB)
        let x = xa.binary_search(&p).unwrap_err();
        let y = yb.binary_search(&q).unwrap_err();
        *berry_map.entry((x, y)).or_insert(0usize) += 1;
    }

    let max = *berry_map.values().max().unwrap();
    let mut min = *berry_map.values().min().unwrap();

    if berry_map.len() < total_piece {
        min = 0;
    }

    println!("{} {}", min, max);
}
