use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        q: usize,
    };

    let mut b2 = vec![BTreeSet::new(); 2_00_001];
    let mut c3 = vec![BTreeSet::new(); 2_00_001];

    for qq in 0..q {
        input! {
            t: usize,
        };
        if t == 1 {
            input! {
                i: usize,
                j: usize
            }
            b2[j].insert((i, qq));
            c3[i].insert(j);
        } else if t == 2 {
            input! {
                i: usize
            }
            for (c, _qq) in b2[i].iter() {
                print!("{} ", *c);
            }
            println!();
        } else {
            input! {
                i: usize
            }
            for b in c3[i].iter() {
                print!("{} ", *b);
            }
            println!();
        }
    }
}
