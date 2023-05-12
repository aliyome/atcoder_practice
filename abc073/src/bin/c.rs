use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut set = HashSet::new();

    for a in a {
        if set.contains(&a) {
            set.remove(&a);
        } else {
            set.insert(a);
        }
    }

    println!("{}", set.len());
}
