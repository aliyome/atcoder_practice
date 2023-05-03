use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(0) += 1;
    }
    let total = count.iter().map(|(_, &v)| v * (v - 1) / 2).sum::<usize>();
    for &ai in &a {
        let c = count[&ai];
        if c > 1 {
            println!("{}", total - c * (c - 1) / 2 + (c - 1) * (c - 2) / 2);
        } else {
            println!("{}", total);
        }
    }
}
