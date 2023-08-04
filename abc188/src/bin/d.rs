use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: isize,
        abc: [(usize, usize, isize); n]
    };

    // 座標圧縮 + いもす法
    let mut map = BTreeMap::new();
    for &(a, b, c) in &abc {
        *map.entry(a).or_insert(0) += c;
        *map.entry(b + 1).or_insert(0) -= c;
    }
    // 累積和
    let mut pre = 0;
    for (_, v) in map.iter_mut() {
        *v += pre;
        pre = *v;
    }
    // 出力
    let mut ans = 0;
    let mut pre_cost = 0;
    let mut pre_day = 0;
    for (&day, &v) in map.iter() {
        let day = day as isize;
        ans += pre_cost.min(c) * (day - pre_day);
        pre_cost = v;
        pre_day = day;
    }

    println!("{}", ans);
}
