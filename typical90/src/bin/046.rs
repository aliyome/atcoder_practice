use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n]
    }

    let mut map_a = HashMap::new();
    let mut map_b = HashMap::new();
    let mut map_c = HashMap::new();
    for i in 0..n {
        *map_a.entry(a[i] % 46).or_insert(0) += 1usize;
        *map_b.entry(b[i] % 46).or_insert(0) += 1usize;
        *map_c.entry(c[i] % 46).or_insert(0) += 1usize;
    }

    let mut ans = 0usize;
    for (a, a_cnt) in map_a.iter() {
        for (b, b_cnt) in map_b.iter() {
            for (c, c_cnt) in map_c.iter() {
                if (a + b + c) % 46 == 0 {
                    ans += a_cnt * b_cnt * c_cnt;
                }
            }
        }
    }

    println!("{}", ans);
}
