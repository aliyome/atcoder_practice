use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    };

    a.insert(0, 0);
    b.insert(0, 0);
    c.insert(0, 0);

    let mut map = HashMap::new();
    for i in 1..=n {
        let c = c[i];
        *map.entry(b[c]).or_insert(0) += 1;
    }

    let mut ans = 0usize;
    for i in 1..=n {
        let a = a[i];
        if let Some(b_cnt) = map.get(&a) {
            ans += b_cnt;
        }
        // for j in 0..n {
        //     let c = c[j];
        //     let b = b[c - 1];
        //     if a == b {
        //         ans += 1;
        //     }
        // }
    }

    println!("{}", ans);
}
