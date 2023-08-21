use std::{cmp::Reverse, collections::HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut map = HashMap::new();
    for s in s.iter() {
        *map.entry(s.clone()).or_insert(0) += 1;
    }

    let mut max = 0;
    for (_, v) in map.iter() {
        max = max.max(*v);
    }

    let mut list = vec![];
    for (k, v) in map.iter() {
        if *v == max {
            list.push(k.clone());
        }
    }

    list.sort();
    for s in list.iter() {
        println!("{}", s);
    }
}
