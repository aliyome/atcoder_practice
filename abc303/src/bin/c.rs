use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: isize,
        k: isize,
        s: Chars,
        items: [(isize, isize); m],
    }

    let mut item_set = HashSet::new();
    for (x, y) in items {
        item_set.insert((x, y));
    }

    let mut pos = (0isize, 0isize);
    let mut count = 0usize;
    for c in s {
        let mut v = (0, 0);
        match c {
            'U' => {
                v = (0, 1);
            }
            'D' => {
                v = (0, -1);
            }
            'L' => {
                v = (-1, 0);
            }
            'R' => {
                v = (1, 0);
            }
            _ => unreachable!(),
        }

        pos.0 += v.0;
        pos.1 += v.1;
        h -= 1;

        if h < 0 {
            println!("No");
            return;
        }

        if h < k && item_set.contains(&pos) {
            h = k;
            item_set.remove(&pos);
        }
        count += 1;

        if count == n {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
