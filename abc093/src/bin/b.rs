use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };

    let mut set = HashSet::new();
    let mut kk = 0;
    for i in a..=b {
        if kk > k {
            break;
        }
        set.insert(i);
        kk += 1;
    }

    let mut kk = 0;
    for i in (a..=b).rev() {
        if kk > k {
            break;
        }
        set.insert(i);
        kk += 1;
    }

    let mut ans: Vec<usize> = set.into_iter().collect();
    ans.sort();

    for i in ans {
        println!("{}", i);
    }
}
