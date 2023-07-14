use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut set_raw = HashSet::new();
    let mut set_ex = HashSet::new();
    for s in s.iter() {
        if s[0] == '!' {
            let x = s[1..].iter().collect::<String>();
            if set_raw.contains(&x) {
                println!("{}", x);
                return;
            }
            set_ex.insert(x);
        } else {
            let x = s[0..].iter().collect::<String>();
            if set_ex.contains(&x) {
                println!("{}", x);
                return;
            }
            set_raw.insert(x);
        }
    }

    println!("satisfiable");
}
