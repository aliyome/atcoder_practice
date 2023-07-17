use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = 0;
    for i in 1..n {
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        s[..i].iter().for_each(|&c| {
            left.insert(c);
        });
        s[i..].iter().for_each(|&c| {
            right.insert(c);
        });
        ans = ans.max(left.intersection(&right).count());
    }

    println!("{}", ans);
}
