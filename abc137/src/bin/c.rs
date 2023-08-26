use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    let mut count = vec![HashMap::new(); n];
    for i in 0..n {
        for &c in &s[i] {
            *count[i].entry(c).or_insert(0) += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if count[i] == count[j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
