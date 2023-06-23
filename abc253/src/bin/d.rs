use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize, // <=10^9
        a: usize, // <=10^9
        b: usize, // <=10^9
    };

    if a == 1 || b == 1 {
        println!("0");
        return;
    }

    let mut set = BTreeSet::new();
    for i in 1..=n {
        set.insert(i);
    }

    let mut set_ab = BTreeSet::new();
    for i in (a..=n).step_by(a) {
        set_ab.insert(i);
    }
    for i in (b..=n).step_by(b) {
        set_ab.insert(i);
    }

    println!("{}", set.difference(&set_ab).into_iter().sum::<usize>());
}
