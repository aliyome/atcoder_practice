use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut set = HashSet::new();
    for i in 1..=n {
        set.insert(i);
    }
    for _ in 0..k {
        input! {
            d: usize,
            a: [usize; d]
        }

        for &a in &a {
            set.remove(&a);
        }
    }
    println!("{}", set.len());
}
