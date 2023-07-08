use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
      q: usize,
    }

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
          t: usize
        }
        match t {
            1 => {
                input! {
                  x: usize
                }
                set.insert(x);
            }
            2 => {
                input! {
                  x: usize
                }
                set.remove(&x);
            }
            3 => {
                input! {
                  x: usize
                }
                if let Some(&y) = set.range(x..).next() {
                    println!("{}", y);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
