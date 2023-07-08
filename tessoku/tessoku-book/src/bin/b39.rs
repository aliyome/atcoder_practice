use std::collections::{BinaryHeap, VecDeque};

use proconio::input;

fn main() {
    input! {
      n: usize,
      d: usize,
      mut xy: [(usize, usize); n]
    }

    // 着手可能日付でAscソート
    xy.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut deq = VecDeque::new();
    for (x, y) in xy {
        deq.push_back((x, y));
    }

    // 優先度付きキュー
    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    for day in 1..=d {
        // 着手可能日付の仕事を優先度付きキューに追加
        while let Some((x, y)) = deq.pop_front() {
            if x <= day {
                heap.push(y);
            } else {
                deq.push_front((x, y));
                break;
            }
        }

        // 仕事を優先度付きキューから取り出す
        if let Some(y) = heap.pop() {
            ans += y;
        }
    }

    println!("{}", ans);
}
