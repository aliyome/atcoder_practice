use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (&k, &v) in map.iter() {
        if k > v {
            ans += v;
        } else {
            ans += v - k;
        }
    }

    println!("{}", ans);
}
