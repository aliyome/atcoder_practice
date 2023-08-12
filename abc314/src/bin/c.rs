use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [usize; n]
    };

    let mut list = vec![VecDeque::new(); m];
    for i in 0..n {
        list[c[i] - 1].push_back(i);
    }

    for i in 0..m {
        if let Some(v) = list[i].pop_back() {
            list[i].push_front(v);
        }
    }

    for i in 0..n {
        print!("{}", s[list[c[i] - 1].pop_front().unwrap()]);
    }
}
