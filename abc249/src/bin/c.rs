use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n]
    };

    let mut ans = 0;
    for i in 0..=1 << n {
        let mut map = HashMap::new();
        for j in 0..n {
            if i & 1 << j != 0 {
                for &c in &s[j] {
                    *map.entry(c).or_insert(0usize) += 1;
                }
            }
        }
        let mut count = 0;
        for (_, v) in map {
            if v == k {
                count += 1;
            }
        }
        ans = ans.max(count);
    }

    println!("{}", ans);
}
