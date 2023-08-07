use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: isize,
        k: isize,
        s: Chars,
        xy: [(isize, isize); m],
    }

    let mut items = HashSet::new();
    for &(x, y) in &xy {
        items.insert((x, y));
    }

    let mut pos = (0, 0);
    for &c in &s {
        h -= 1;
        if c == 'R' {
            pos.0 += 1;
        } else if c == 'L' {
            pos.0 -= 1;
        } else if c == 'U' {
            pos.1 += 1;
        } else if c == 'D' {
            pos.1 -= 1;
        }

        if h < 0 {
            println!("No");
            return;
        }

        if items.contains(&pos) && h < k {
            h = k;
            items.remove(&pos);
        }
    }

    println!("Yes");
}
