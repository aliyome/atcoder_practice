use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut next_in_line = HashMap::new();

    for (i, &person) in a.iter().enumerate() {
        next_in_line.insert(person, i as isize + 1);
    }

    let mut current = *next_in_line.get(&-1).unwrap();

    for _ in 0..n {
        print!("{} ", current);
        current = *next_in_line.get(&current).unwrap_or(&0);
    }
    println!();
}
