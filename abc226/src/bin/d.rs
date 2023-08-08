use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let v = (xy[i].0 - xy[j].0, xy[i].1 - xy[j].1);
            let nv = gcd(v.0, v.1);
            let v = (v.0 / nv, v.1 / nv);
            set.insert(v);
        }
    }
    println!("{}", set.len() * 2);
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
