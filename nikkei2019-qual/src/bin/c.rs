use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    let mut skip = HashSet::new();
    let mut turn = 0;
    let mut takahashi = 0;
    let mut aoki = 0;
    for _ in 0..n {
        let mut min = 10_000_000_000;
        let mut min_i = 0;
        for i in 0..n {
            let (a, b) = ab[i];
            if skip.contains(&i) {
                continue;
            }
            if turn == 0 {
                // takahashi
                if b < min {
                    min = b;
                    min_i = i;
                }
            } else {
                // aoki
                if a < min {
                    min = a;
                    min_i = i;
                }
            }
        }
        if turn == 0 {
            takahashi += ab[min_i].0;
        } else {
            aoki += ab[min_i].1;
        }

        skip.insert(min_i);
        turn = 1 - turn;
    }

    println!("{}", takahashi as isize - aoki as isize);
}
