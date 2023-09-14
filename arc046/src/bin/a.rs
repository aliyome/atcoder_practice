use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut x = 0;
    for _ in 0..n {
        loop {
            x += 1;
            let mut set = HashSet::new();
            for c in format!("{}", x).chars() {
                set.insert(c);
            }
            if set.len() == 1 {
                break;
            }
        }
    }
    println!("{}", x);
}
