use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        w: [Chars; n]
    };

    let mut set = HashSet::new();
    for i in 0..n {
        if i > 0 && w[i][0] != w[i - 1][w[i - 1].len() - 1] {
            println!("No");
            return;
        }
        if set.contains(&w[i].iter().collect::<String>()) {
            println!("No");
            return;
        }

        set.insert(w[i].iter().collect::<String>());
    }

    println!("Yes");
}
