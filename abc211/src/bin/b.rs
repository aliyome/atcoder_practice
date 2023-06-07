use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
        s4: Chars,
    };

    let mut set = HashSet::new();
    set.insert(s1);
    set.insert(s2);
    set.insert(s3);
    set.insert(s4);

    if set.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
