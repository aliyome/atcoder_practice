use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    let mut hachi = vec![];
    for i in (8..1000).step_by(8) {
        let chars = format!("{:03}", i).chars().collect_vec();
        let mut map = HashMap::new();
        for c in chars {
            *map.entry(c).or_insert(0) += 1;
        }
        hachi.push(map);
    }

    if s.len() < 4 {
        s = format!(
            "{:03}",
            s.iter().collect::<String>().parse::<i32>().unwrap()
        )
        .chars()
        .collect_vec();
    }

    let mut map = HashMap::new();
    for &c in &s {
        *map.entry(c).or_insert(0) += 1;
    }

    for h in hachi {
        let mut ok = true;
        for (k, v) in h {
            if !map.contains_key(&k) || map[&k] < v {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
