use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    };

    let mut set = HashSet::new();
    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            set.insert(i);
            set.insert(n / i);
        }
    }

    let mut list = set.iter().collect::<Vec<_>>();
    list.sort();
    for i in list {
        println!("{}", i);
    }
}
