use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut rev = HashMap::new();

    // O(N)
    for i in 0..n {
        let a = a[i];
        let next_1 = (i + 1) * 2;
        let next_2 = (i + 1) * 2 + 1;

        rev.insert(next_1, a);
        rev.insert(next_2, a);
    }

    // O(N^2)
    for k in 1..=2 * n + 1 {
        if !rev.contains_key(&k) {
            println!("0");
            continue;
        }
        let mut count = 0;
        let mut child = k;
        while let Some(parent) = rev.get(&child) {
            count += 1;
            child = *parent;
        }
        println!("{}", count);
    }
}
