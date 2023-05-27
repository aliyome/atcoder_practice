use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: i64,
        k: i64,
        s: String,
        items: [(i64, i64); m],
    }

    let mut items_set = HashSet::new();
    for item in items {
        items_set.insert(item);
    }

    let mut position = (0, 0);
    for direction in s.chars() {
        h -= 1;
        match direction {
            'R' => position.0 += 1,
            'L' => position.0 -= 1,
            'U' => position.1 += 1,
            'D' => position.1 -= 1,
            _ => unreachable!(),
        }
        if h < 0 {
            println!("No");
            return;
        }
        if items_set.contains(&position) && h < k {
            h = k;
        }
    }
    println!("Yes");
}
