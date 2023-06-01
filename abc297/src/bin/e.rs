use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n]
    };

    // min-heap
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(0));

    let mut used = HashSet::new();
    for _ in 0..k {
        let p = heap.pop().unwrap().0;

        for &a in &a {
            let next = p + a;
            if !used.contains(&next) {
                heap.push(Reverse(next));
                used.insert(next);
            }
        }
    }

    println!("{}", heap.pop().unwrap().0);
}
