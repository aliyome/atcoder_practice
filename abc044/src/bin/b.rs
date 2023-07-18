use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        w: Chars,
    };

    let mut map = HashMap::new();
    for &c in &w {
        *map.entry(c).or_insert(0) += 1usize;
    }

    for (_, &v) in map.iter() {
        if v % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
