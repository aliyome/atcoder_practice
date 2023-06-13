use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize
    };

    a.insert(0, 0);

    // 愚直にやるとO(QN) 10^10 で TLE
    // O(Q) 10^5
    let mut base = 10usize.pow(10);
    let mut adds = HashMap::new();
    for _ in 0..q {
        input! {
            t: usize
        };
        if t == 1 {
            input! {
                x: usize
            };
            // // O(N) 10^5
            // for i in 1..n {
            //     a[i] = x;
            // }
            base = x;
            adds = HashMap::new();
        } else if t == 2 {
            input! {
                i: usize,
                x: usize,
            };
            // a[i] += x;
            *adds.entry(i).or_insert(0) += x;
        } else {
            input! {
                i: usize,
            }
            if base == 10usize.pow(10) {
                println!("{}", a[i] + adds.get(&i).unwrap_or(&0));
            } else {
                println!("{}", base + adds.get(&i).unwrap_or(&0));
            }
        }
    }
}
