use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut set = HashSet::new();
    for a in 2..=(n as f64).sqrt() as usize {
        let mut x = a * a;
        while x <= n {
            set.insert(x);
            x *= a;
        }
    }
    println!("{}", n - set.len());
}
