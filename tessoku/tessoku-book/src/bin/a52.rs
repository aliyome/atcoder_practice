use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
      q: usize,
    }

    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
          t: usize
        }
        match t {
            1 => {
                input! {
                  s: String
                }
                queue.push_back(s);
            }
            2 => {
                if let Some(s) = queue.front() {
                    println!("{}", s);
                }
            }
            3 => {
                queue.pop_front();
            }
            _ => unreachable!(),
        }
    }
}
