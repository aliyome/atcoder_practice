use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    };

    let mut map = HashMap::new();
    *map.entry(a).or_insert(0) += 1;
    *map.entry(b).or_insert(0) += 1;
    *map.entry(c).or_insert(0) += 1;
    *map.entry(d).or_insert(0) += 1;
    *map.entry(e).or_insert(0) += 1;

    let mut n2 = false;
    let mut n3 = false;
    for (_, &v) in map.iter() {
        if v == 2 {
            n2 = true;
        }
        if v == 3 {
            n3 = true;
        }
    }
    if n2 && n3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
