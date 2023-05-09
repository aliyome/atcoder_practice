use std::collections::HashSet;
use std::iter::FromIterator;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    };

    // M個の集合
    let mut s = vec![HashSet::new(); m];
    for i in 0..m {
        input! {
            c: i64,
            a: [i64; c],
        };
        let set: HashSet<i64> = HashSet::from_iter(a);
        s[i] = set;
    }

    // 答えとなるxの和集合
    let x: HashSet<i64> = HashSet::from_iter(1..=n as i64);

    let mut count = 0;
    // 全探索
    for i in 1..(1 << m) {
        // 選んだ集合
        let mut t = vec![];
        for j in 0..m {
            if i & (1 << j) == 0 {
                continue;
            }
            t.push(&s[j]);
        }
        // 選んだ集合が条件を満たすか
        let mut y = t[0].clone();
        for j in 1..t.len() {
            y = y.union(&t[j]).cloned().collect();
        }
        if x == y {
            count += 1;
        }
        // println!("{:?}", y);
    }

    println!("{}", count);
}
