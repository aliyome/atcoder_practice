use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut s_cols = HashSet::new();
    let mut t_cols = HashSet::new();
    for j in 0..w {
        let mut s_col = vec![];
        let mut t_col = vec![];
        for i in 0..h {
            s_col.push(s[i][j]);
            t_col.push(t[i][j]);
        }
        s_cols.insert(s_col);
        t_cols.insert(t_col);
    }

    if s_cols == t_cols {
        println!("Yes");
    } else {
        println!("No");
    }
}
