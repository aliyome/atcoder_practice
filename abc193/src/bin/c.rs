use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let mut set = HashSet::new();
    for i in 2..=10u64.pow(9) {
        if i > n {
            break;
        }
        let mut x = i * i;
        while x <= n {
            set.insert(x);
            x *= i;
        }
    }

    println!("{}", n - set.len() as u64);
}
