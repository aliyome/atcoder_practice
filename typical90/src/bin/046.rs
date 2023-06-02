use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n]
    }

    let mut a_map = HashMap::new();
    let mut b_map = HashMap::new();
    let mut c_map = HashMap::new();
    for i in 0..n {
        *a_map.entry(a[i] % 46).or_insert(0usize) += 1;
        *b_map.entry(b[i] % 46).or_insert(0usize) += 1;
        *c_map.entry(c[i] % 46).or_insert(0usize) += 1;
    }

    let mut ans = 0usize;
    for (a, a_cnt) in a_map {
        for (b, b_cnt) in &b_map {
            for (c, c_cnt) in &c_map {
                if (a + b + c) % 46 == 0 {
                    ans += a_cnt * b_cnt * c_cnt;
                }
            }
        }
    }

    println!("{}", ans);
}
