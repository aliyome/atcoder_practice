use std::{cmp::Ordering, collections::BinaryHeap};

use proconio::input;

// https://stackoverflow.com/a/39985950
#[derive(PartialEq, Debug, Clone)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [f64; n]
    };

    let mut heap = BinaryHeap::new();
    for &a in &a {
        heap.push(MinNonNan(a));
    }

    // すべての丸太の長さが x 以内に収まるか
    let is_ok = |x: usize| {
        // K回分割を繰り返すのではなく
        // x以下に分割するには何等分する必要があるかを考えて、それがK以下かどうかを判定する
        let mut count = 0;
        for &a in &a {
            let y = (a as usize - 1) / x;
            count += y;
        }
        count <= k
    };

    let mut ng = 0;
    let mut ok = 10usize.pow(9);
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
