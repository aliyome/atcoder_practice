use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    };

    let mut sell = BTreeSet::new();
    let mut buy = BTreeSet::new();
    for i in 0..n {
        sell.insert((a[i], i as isize));
    }
    for i in 0..m {
        buy.insert((b[i], i as isize));
    }

    let mut ng = 0;
    let mut ok = 1_000_000_000;
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        let sell_cnt = sell.range(..=(mid, 1_000_000_000)).count();
        let buy_cnt = buy.range((mid, -1)..).count();
        if buy_cnt <= sell_cnt {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
