use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        f: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    for &f in &f {
        heap.push(f);
    }

    let mut ans = 0usize;
    while heap.len() > d {
        // 運賃の高い方からd日分popする
        let mut temp = 0usize;
        for _ in 0..d {
            temp += heap.pop().unwrap();
        }
        if temp > p {
            ans += p;
        } else {
            ans += temp;
        }
    }

    let rest = heap.iter().sum::<usize>();
    if rest > p {
        ans += p;
    } else {
        ans += rest;
    }

    println!("{}", ans);
}
