use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(Reverse(a[i]));
    }
    for i in 0..m {
        heap.push(Reverse(b[i]));
    }

    let mut time = 0;
    let mut count = 0;
    while let Some(Reverse(x)) = heap.pop() {
        if time + x > k {
            break;
        }
        time += x;
        count += 1;
    }

    println!("{}", count);
}
