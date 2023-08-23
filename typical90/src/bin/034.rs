use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut map = HashMap::new();
    let mut q = VecDeque::new();
    let mut ans = 0;

    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
        q.push_back(a);
        if map.len() <= k {
            ans = ans.max(q.len());
        } else {
            while let Some(x) = q.pop_front() {
                map.entry(x).and_modify(|f| *f -= 1);
                if map[&x] == 0 {
                    map.remove(&x);
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
