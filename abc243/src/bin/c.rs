use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: Chars
    };

    let mut map_r = HashMap::new();
    let mut map_l = HashMap::new();
    for i in 0..n {
        if s[i] == 'L' {
            let l = *map_l.entry(xy[i].1).or_insert(0);
            map_l.insert(xy[i].1, xy[i].0.max(l));
        } else {
            let r = *map_r.entry(xy[i].1).or_insert(std::usize::MAX);
            map_r.insert(xy[i].1, xy[i].0.min(r));
        }
    }

    for (&k, &l) in map_l.iter() {
        if let Some(&r) = map_r.get(&k) {
            if r < l {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
