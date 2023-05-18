use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        lr: [(usize, usize); n]
    }

    // // 全探索 O(NW) TLE
    // let mut brick = vec![0; w + 1];
    // for i in 0..n {
    //     let (l, r) = lr[i];
    //     let mut max = 0;
    //     for j in l..=r {
    //         max = max.max(brick[j]);
    //     }
    //     for j in l..=r {
    //         brick[j] = max + 1;
    //     }
    //     println!("{}", max + 1);
    // }

    // 座標圧縮 O(N + N^2)
    let mut set = HashSet::new();
    for i in 0..n {
        let (l, r) = lr[i];
        set.insert(l);
        set.insert(r);
    }
    let mut rank = HashMap::new();
    for (i, v) in set.iter().sorted().enumerate() {
        rank.insert(v, i);
    }
    let mut brick = vec![0; n * 2 + 1];
    for i in 0..n {
        let (l, r) = lr[i];
        let l = *rank.get(&l).unwrap();
        let r = *rank.get(&r).unwrap();
        let mut max = 0;
        for j in l..=r {
            max = max.max(brick[j]);
        }
        for j in l..=r {
            brick[j] = max + 1;
        }
        println!("{}", max + 1);
    }
}
