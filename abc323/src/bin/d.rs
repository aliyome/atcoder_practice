use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        slimes: [(u64, u64); n],
    }

    let mut slimes_map = HashMap::new();

    for (s, c) in slimes {
        *slimes_map.entry(s).or_insert(0) += c;
    }

    let mut keys: Vec<_> = slimes_map.keys().cloned().collect();
    keys.sort_unstable();

    let mut idx = 0;
    while idx < keys.len() {
        let size = keys[idx];
        if let Some(&count) = slimes_map.get(&size) {
            if count >= 2 {
                let combined = count / 2;
                *slimes_map.entry(size * 2).or_insert_with(|| {
                    keys.push(size * 2);
                    0
                }) += combined;
                *slimes_map.entry(size).or_insert(0) -= combined * 2;
            }
        }
        idx += 1;
    }

    let result: u64 = slimes_map.values().sum();

    println!("{}", result);
}
