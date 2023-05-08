use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); n]
    };

    let mut jobs = vec![vec![]; m + 1];
    for (a, b) in ab {
        if a <= m {
            jobs[a].push(b);
        }
    }

    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    for i in 1..=m {
        for &b in &jobs[i] {
            heap.push(b);
        }
        if let Some(b) = heap.pop() {
            ans += b;
        }
    }

    println!("{}", ans);
}
