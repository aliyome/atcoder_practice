use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut s_cols = BTreeMap::new();
    let mut t_cols = BTreeMap::new();
    for j in 0..w {
        let mut s_col = vec![];
        let mut t_col = vec![];
        for i in 0..h {
            s_col.push(s[i][j]);
            t_col.push(t[i][j]);
        }
        *s_cols.entry(s_col).or_insert(0) += 1;
        *t_cols.entry(t_col).or_insert(0) += 1;
    }

    for (k, s_v) in s_cols.iter() {
        match t_cols.get(k) {
            Some(&t_v) => {
                if *s_v != t_v {
                    println!("No");
                    return;
                }
            }
            None => {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
