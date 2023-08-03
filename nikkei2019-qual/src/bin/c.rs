use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(isize, isize); n]
    };

    let mut dishes = vec![];
    for &(a, b) in &ab {
        dishes.push((a + b, a, b));
    }
    dishes.sort_by(|a, b| b.0.cmp(&a.0));

    let mut ans = 0isize;
    for i in 0..n {
        if i % 2 == 0 {
            ans += dishes[i].1;
        } else {
            ans -= dishes[i].2;
        }
    }
    println!("{}", ans);
}
