use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: isize,
      x: isize,
      mut a: Chars
    }

    let mut q = VecDeque::new();
    q.push_back(x - 1);
    a[x as usize - 1] = '@';
    while let Some(c) = q.pop_front() {
        if c - 1 >= 0 && a[c as usize - 1] == '.' {
            a[c as usize - 1] = '@';
            q.push_back(c - 1);
        }
        if c + 1 < n && a[c as usize + 1] == '.' {
            a[c as usize + 1] = '@';
            q.push_back(c + 1);
        }
    }

    println!("{}", a.iter().collect::<String>());
}
