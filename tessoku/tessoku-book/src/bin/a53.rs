use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
      q: usize,
    }

    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        input! {
          t: usize
        }
        match t {
            1 => {
                input! {
                  x: isize
                }
                heap.push(Reverse(x));
            }
            2 => {
                if let Some(x) = heap.peek() {
                    println!("{}", x.0);
                }
            }
            3 => {
                heap.pop();
            }
            _ => unreachable!(),
        }
    }
}
