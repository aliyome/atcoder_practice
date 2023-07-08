use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize, // <= 3 x 10^5
        k: i64, // <= 10^9
        mut ab: [(i64, i64); n] // a,b, <= 10^9
    };

    // 全量
    let mut sum = ab.iter().fold(0, |acc, (_, b)| acc + b);

    if sum <= k {
        println!("1");
        return;
    }

    // 日毎にまとめる
    let mut map = BTreeMap::new();
    for &(a, b) in &ab {
        *map.entry(a).or_insert(0) += b;
    }

    // K以下になる日を探す
    for (a, b) in map {
        sum -= b;
        if sum <= k {
            println!("{}", a + 1);
            return;
        }
    }

    // ここには来ないはず
    ab.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{}", ab[n - 1].0 + 1);
}
