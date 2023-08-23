use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
    };

    let mut flip = false;
    let mut deq = VecDeque::new();

    for &c in &s {
        deq.push_back(c);
    }

    for _ in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            flip = !flip;
        } else {
            input! {
                f: usize,
                c: char
            }
            if flip {
                if f == 1 {
                    deq.push_back(c);
                } else {
                    deq.push_front(c);
                }
            } else {
                if f == 1 {
                    deq.push_front(c);
                } else {
                    deq.push_back(c);
                }
            }
        }
    }

    if flip {
        deq = deq.into_iter().rev().collect();
    }

    println!("{}", deq.into_iter().collect::<String>());
}
