use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let mut possible_values = HashSet::new();
    for a in 1..=1000 {
        for b in 1..=1000 {
            let area = 4 * a * b + 3 * a + 3 * b;
            if area <= 1000 {
                possible_values.insert(area);
            } else {
                break;
            }
        }
    }

    let mut count = 0;
    for &guess in &s {
        if !possible_values.contains(&guess) {
            count += 1;
        }
    }

    println!("{}", count);
}
