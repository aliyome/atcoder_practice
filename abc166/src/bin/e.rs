use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        heights: [i64; n],
    }

    let mut count_map: HashMap<i64, i64> = HashMap::new();

    for i in 0..n {
        let key = i as i64 - heights[i];
        let count = count_map.entry(key).or_insert(0);
        *count += 1;
    }

    let mut result = 0;

    for i in 0..n {
        let key = i as i64 + heights[i];
        if let Some(count) = count_map.get(&key) {
            result += count;
        }
    }

    println!("{}", result);
}
