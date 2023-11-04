use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }

    let mut rows = vec![HashSet::new(); n];
    let mut cols = vec![HashSet::new(); n];
    for i in 0..n {
        for j in 0..n {
            rows[i].insert(r[i]);
            cols[i].insert(c[i]);
        }
    }

    let is_valid = |rows: &Vec<HashSet<char>>, cols: &Vec<HashSet<char>>| -> bool {
        for i in 0..n {
            for j in 0..n {
                if rows[i].intersection(&cols[j]).next().is_none() {
                    return false;
                }
            }
        }
        true
    };

    if is_valid(&rows, &cols) {
        println!("Yes");
        for i in 0..n {
            for j in 0..n {
                print!("{}", rows[i].intersection(&cols[j]).next().unwrap());
            }
            println!();
        }
        return;
    }
    println!("No");
}
