use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize, // <= 3 x 10^5
        k: usize, // <= 10^9
        mut ab: [(usize, usize); n] // a,b, <= 10^9
    };

    // 同じ日はまとめる
    let mut abmap = BTreeMap::new();
    for (a, b) in ab {
        let entry = abmap.entry(a).or_insert(0);
        *entry += b;
    }

    // 初日の数
    let mut sum = abmap.iter().map(|(_, b)| *b).sum::<usize>();

    // 初日の数がkを超えていたら、初日の数をkにする
    if sum <= k {
        println!("1");
        return;
    }

    // 減らしていく
    for (a, b) in abmap.iter() {
        sum -= *b;
        if sum <= k {
            println!("{}", *a + 1);
            return;
        }
    }
}
