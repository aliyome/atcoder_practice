use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        slimes: [(u64, u64); n],
    }

    let mut count = HashMap::new();

    for (size, num) in slimes {
        let mut num = num;
        let mut size = size;
        while num > 0 {
            if num & 1 == 1 {
                *count.entry(size).or_insert(0usize) += 1;
            }
            size *= 2;
            num >>= 1;
        }
    }

    let mut result = 0;

    for &val in count.values() {
        let mut bits = val;
        while bits > 0 {
            result += bits & 1;
            bits >>= 1;
        }
    }

    println!("{}", result);
}
