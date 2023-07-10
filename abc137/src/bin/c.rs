use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    let mut map = HashMap::new();
    for i in 0..n {
        let mut x = s[i].clone();
        x.sort();
        *map.entry(x).or_insert(0usize) += 1;
    }

    let mut ans = 0;
    for (_, v) in map {
        if v > 1 {
            ans += v * (v - 1) / 2;
        }
    }

    println!("{}", ans);
}
