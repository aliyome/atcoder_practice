use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        let win = s[i].iter().filter(|&c| *c == 'o').count();
        heap.push((win, Reverse(i)));
    }

    while let Some((_, Reverse(i))) = heap.pop() {
        print!("{} ", i + 1);
    }
}
