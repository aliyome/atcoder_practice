use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize, // <= 3 x 10^5
        k: i64, // <= 10^9
        mut ab: [(i64, i64); n] // a,b, <= 10^9
    };

    // 全量
    let mut sum = ab.iter().fold(0, |acc, (a, b)| acc + b);

    // 日付でソート
    ab.sort_by(|a, b| a.0.cmp(&b.0));

    for &(a, b) in &ab {
        if sum <= k {
            println!("{}", a);
            return;
        }
        sum -= b;
        if sum <= k {
            println!("{}", a + 1);
            return;
        }
    }

    println!("{}", ab[n - 1].0 + 1);
}
