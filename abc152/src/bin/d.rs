use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut map = HashMap::new();
    for i in 1..=n {
        let s = i.to_string().chars().collect_vec();
        *map.entry((*s.first().unwrap(), *s.last().unwrap()))
            .or_insert(0) += 1;
    }

    let mut ans = 0;
    for (k, v) in map.iter() {
        for (kk, vv) in map.iter() {
            if k.0 == kk.1 && k.1 == kk.0 {
                ans += v * vv;
            }
        }
    }

    println!("{}", ans);
}
