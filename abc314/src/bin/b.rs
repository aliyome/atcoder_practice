use std::collections::{BTreeMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut a = vec![HashSet::new(); n];
    for i in 0..n {
        input! {
            c: usize,
            ai: [usize; c]
        };
        for j in 0..c {
            a[i].insert(ai[j]);
        }
    }

    input! {
        x: usize
    };

    let mut map = BTreeMap::new();
    for i in 0..n {
        if a[i].contains(&x) {
            map.entry(a[i].len()).or_insert(vec![]).push(i + 1);
        }
    }
    if map.len() == 0 {
        println!("0");
        println!();
    } else {
        let (_, list) = map.iter().next().unwrap();
        println!("{}", list.len());
        println!(
            "{}",
            list.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
