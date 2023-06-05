use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
      a: usize,
      b: usize,
    }

    let mut set = HashSet::new();
    for i in 1..=100 {
        if 100 % i == 0 {
            set.insert(i);
        }
    }

    for i in a..=b {
        if set.contains(&i) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
