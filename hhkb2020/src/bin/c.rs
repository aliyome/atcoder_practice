use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut set = BTreeSet::new();
    for i in 0..=200_000 {
        set.insert(i);
    }

    for &p in &p {
        set.remove(&p);
        println!("{}", set.iter().next().unwrap());
    }
}
