use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
    }

    let mut hash_without_1 = HashSet::new();
    let mut hash_with_1 = HashSet::new();

    for _ in 0..n {
        input! {
            s: Chars,
        };
        if s[0] != '!' {
            let s = s.iter().collect::<String>();
            hash_without_1.insert(s.clone());
            if hash_with_1.contains(&s) {
                println!("{}", s);
                return;
            }
        } else {
            let s = s.iter().skip(1).collect::<String>();
            hash_with_1.insert(s.clone());
            if hash_without_1.contains(&s) {
                println!("{}", s);
                return;
            }
        }
    }

    println!("satisfiable");
}
