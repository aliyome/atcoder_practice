use std::collections::{BinaryHeap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut ng = (n + 1) / 2 + 1;
    let mut ok = 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if can_form_pyramid(&a, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn can_form_pyramid(a: &[usize], size: usize) -> bool {
    let mut candidates = BinaryHeap::new();
    for (i, &val) in a.iter().enumerate() {
        if val >= size && i >= size - 1 && i + size <= a.len() {
            candidates.push((val, i));
        }
    }
    eprintln!("size: {}, candidates: {:?}", size, candidates);

    if candidates.is_empty() {
        return false;
    }

    while let Some((_val, i)) = candidates.pop() {
        for j in 0..size {
            if a[i + j] < size - j {
                return false;
            }
            if a[i - j] < size - j {
                return false;
            }
        }
    }

    true
}
