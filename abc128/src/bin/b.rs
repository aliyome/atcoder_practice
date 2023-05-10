use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n],
    };

    let mut sorted = sp.clone();
    sorted.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    for (s, p) in sorted {
        for i in 0..n {
            if sp[i].0 == s && sp[i].1 == p {
                println!("{}", i + 1);
                break;
            }
        }
    }
}
