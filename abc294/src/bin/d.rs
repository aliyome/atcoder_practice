use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashSet},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut uncalled = BTreeSet::new();
    for i in 1..=n {
        uncalled.insert(i);
    }

    let mut called = BTreeSet::<usize>::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                let x = *uncalled.iter().next().unwrap();
                uncalled.remove(&x);
                called.insert(x);
            }
            2 => {
                input! {
                    x: usize
                }
                called.remove(&x);
            }
            3 => {
                println!("{}", called.iter().next().unwrap());
            }
            _ => unreachable!(),
        }
    }
}
