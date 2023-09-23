use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: u64,
        main_dishes: [u64; n],
        side_dishes: [u64; m],
    }

    // BinaryHeapは最大ヒープなので、-1を掛けることで最小値を取り出せるようにする。
    let mut heap = BinaryHeap::new();
    for &main in &main_dishes {
        for &side in &side_dishes {
            heap.push(-(std::cmp::min(main + side, p) as i64));
        }
    }

    let mut total_price = 0u64;
    for _ in 0..n * m {
        if let Some(price) = heap.pop() {
            total_price += -price as u64;
        }
    }

    println!("{}", total_price);
}
