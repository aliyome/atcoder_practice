use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    let mut count = HashMap::new();
    let mut ans = 0usize;
    for i in 0..n {
        let mut s = s[i].clone();
        s.sort();

        if count.contains_key(&s) {
            ans += count[&s];
        }

        *count.entry(s.clone()).or_insert(0) += 1;
    }

    println!("{}", ans);
}
