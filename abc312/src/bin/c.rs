use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    };

    let mut heap = BinaryHeap::new();
    for &a in &a {
        heap.push((Reverse(a), 'a'));
    }
    for &b in &b {
        heap.push((Reverse(b + 1), 'b'));
    }

    let mut a = 0;
    let mut b = b.len();
    let mut ans = 0;
    while a < b {
        let (Reverse(price), ab) = heap.pop().unwrap();
        match ab {
            'a' => a += 1,
            'b' => b -= 1,
            _ => unreachable!(),
        }
        ans = price;
    }
    println!("{}", ans);
}
