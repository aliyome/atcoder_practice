use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    };

    let mut map = HashMap::new();
    let mut sum = 0;
    for &a in &a {
        *map.entry(a).or_insert(0usize) += 1;
        sum += a;
    }

    for &(b, c) in &bc {
        if let Some(&v) = map.get(&b) {
            sum -= b * v;
            sum += c * v;
            map.remove(&b);
            *map.entry(c).or_insert(0) += v;
        }
        println!("{}", sum);
    }
}
