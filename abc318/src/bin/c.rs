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

    for &f in f.iter() {
        heap.push(f);
    }

    let mut total_cost = 0;
    while heap.len() >= d {
        let mut temp = 0;
        for _ in 0..d {
            temp += heap.pop().unwrap();
        }
        if temp > p {
            total_cost += p;
        } else {
            total_cost += temp;
        }
    }

    let mut temp = 0;
    while let Some(f) = heap.pop() {
        temp += f;
    }

    if temp > p {
        total_cost += p;
    } else {
        total_cost += temp;
    }

    println!("{}", total_cost);
}
