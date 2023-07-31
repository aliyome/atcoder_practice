use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut map = BTreeMap::new();

    for _ in 1..=q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize
            }
            *map.entry(x).or_insert(0) += 1isize;
        } else if t == 2 {
            input! {
                x: usize,
                c: usize,
            }
            if let Some(v) = map.get_mut(&x) {
                let new_v = *v - (c as isize);
                if new_v <= 0 {
                    map.remove(&x);
                } else {
                    map.insert(x, new_v);
                }
            }
        } else {
            let min = map.keys().next().unwrap();
            let max = map.keys().rev().next().unwrap();

            println!("{}", max - min);
        }
    }
}
