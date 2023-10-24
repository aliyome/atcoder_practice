use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: i64,
        d: [[i64; n]; n]
    }

    // ダイクストラで 0 から n-1 までの社用車の最短経路を求める
    let mut heap = BinaryHeap::new();
    let mut dist_car = vec![std::i64::MAX; n];
    dist_car[0] = 0;
    heap.push((Reverse(0), 0));
    while let Some((Reverse(cost), v)) = heap.pop() {
        if dist_car[v] < cost {
            continue;
        }
        for (u, &d) in d[v].iter().enumerate() {
            if dist_car[u] > dist_car[v] + d * a {
                dist_car[u] = dist_car[v] + d * a;
                heap.push((Reverse(dist_car[u]), u));
            }
        }
    }

    // ダイクストラで n-1 から 0 までの電車の最短経路を求める
    let mut heap = BinaryHeap::new();
    let mut dist_train = vec![std::i64::MAX; n];
    dist_train[n - 1] = 0;
    heap.push((Reverse(0), n - 1));
    while let Some((Reverse(cost), v)) = heap.pop() {
        if dist_train[v] < cost {
            continue;
        }
        for (u, &d) in d[v].iter().enumerate() {
            if dist_train[u] > dist_train[v] + d * b + c {
                dist_train[u] = dist_train[v] + d * b + c;
                heap.push((Reverse(dist_train[u]), u));
            }
        }
    }

    // 0 から n-1 までの最短経路を求める
    let mut ans = std::i64::MAX;
    for i in 0..n {
        ans = ans.min(dist_car[i] + dist_train[i]);
    }

    println!("{}", ans);
}
