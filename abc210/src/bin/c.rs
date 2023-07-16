use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n]
    };

    let mut map = HashMap::new();
    // [0..k) までの値をカウント
    for j in 0..k {
        *map.entry(c[j]).or_insert(0) += 1;
    }

    let mut ans = map.keys().len();
    for i in k..n {
        // 一つ前のを削除
        *map.entry(c[i - k]).or_insert(0) -= 1;
        if map[&c[i - k]] == 0 {
            map.remove(&c[i - k]);
        }
        // 一つ後ろのを追加
        *map.entry(c[i]).or_insert(0) += 1;
        ans = ans.max(map.keys().len());
    }

    println!("{}", ans);
}
