use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        ca: [(usize, [usize; k]); n],
    }

    let add = |a: &Vec<usize>, b: &Vec<usize>| {
        let mut res = vec![0; k];
        for i in 0..k {
            res[i] = (a[i] + b[i]).min(p);
        }
        res
    };

    let mut dp = HashMap::new();
    dp.insert(vec![0; k], 0);

    for (c, a) in &ca {
        let mut next_dp = dp.clone();
        for (ks, cost) in dp.iter() {
            let added = add(ks, a);
            if next_dp.contains_key(&added) {
                let prev = *next_dp.get(&added).unwrap();
                next_dp.insert(added, prev.min(cost + c));
            } else {
                next_dp.insert(added, cost + c);
            }
        }
        dp = next_dp;
    }

    let mut min = std::usize::MAX;
    for (k, v) in dp.iter() {
        if k.iter().all(|&x| x >= p) {
            min = min.min(*v);
        }
    }

    if min == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", min);
    }
}
