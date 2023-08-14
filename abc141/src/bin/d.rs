use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    };

    let mut heap = BinaryHeap::new();
    for &a in &a {
        heap.push(a);
    }

    for _ in 0..m {
        let max = heap.pop().unwrap();
        heap.push(max / 2);
    }

    println!("{}", heap.iter().sum::<usize>());
}
