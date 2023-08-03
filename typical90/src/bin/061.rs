use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q]
    }

    let mut queue = VecDeque::new();
    for (t, x) in tx {
        match t {
            1 => queue.push_front(x),
            2 => queue.push_back(x),
            3 => println!("{}", queue[x - 1]),
            _ => unreachable!(),
        }
    }
}
