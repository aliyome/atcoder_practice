use std::collections::{BTreeSet, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    };

    let mut rows = BTreeSet::new();
    let mut cols = BTreeSet::new();
    for i in 0..h {
        rows.insert(i);
    }
    for j in 0..w {
        cols.insert(j);
    }
    loop {
        let mut updated = false;

        let mut cols_to_remove = HashSet::new();
        for &j in cols.iter() {
            let mut set = HashSet::new();
            let mut count = 0usize;
            for &i in rows.iter() {
                set.insert(c[i][j]);
                count += 1;
            }
            if set.len() == 1 && count > 1 {
                updated = true;
                cols_to_remove.insert(j);
            }
        }
        for &j in cols_to_remove.iter() {
            cols.remove(&j);
        }

        let mut rows_to_remove = HashSet::new();
        for &i in rows.iter() {
            let mut set = HashSet::new();
            let mut count = 0usize;
            for &j in cols.iter() {
                set.insert(c[i][j]);
                count += 1;
            }
            if set.len() == 1 && count > 1 {
                updated = true;
                rows_to_remove.insert(i);
            }
        }
        for &i in rows_to_remove.iter() {
            rows.remove(&i);
        }

        if !updated {
            break;
        }
    }

    // 残ったクッキーの数を数える
    println!("{}", rows.len() * cols.len());

    // // DEBUG
    // for &i in rows.iter() {
    //     for &j in cols.iter() {
    //         print!("{}", c[i][j]);
    //     }
    //     println!();
    // }
}
