use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut heap = BinaryHeap::new();
    for &a in &a {
        heap.push(a);
    }

    let mut ans = 0;
    while let Some(x) = heap.pop() {
        if x % 2 == 1 {
            continue;
        }

        ans += 1;
        heap.push(x / 2);
    }

    println!("{}", ans);
}
