use std::collections::{BTreeSet, HashMap, HashSet};

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

    let mut row_map = vec![vec![0usize; 26]; h];
    let mut col_map = vec![vec![0usize; 26]; w];
    for i in 0..h {
        for j in 0..w {
            row_map[i][c[i][j] as usize - 'a' as usize] += 1;
            col_map[j][c[i][j] as usize - 'a' as usize] += 1;
        }
    }
    loop {
        let mut updated = false;

        let mut rows_to_remove = HashSet::new();
        for &i in rows.iter() {
            let mut letters = 0;
            let mut count = 0usize;
            let mut letter = 0usize;
            for k in 0..26 {
                if row_map[i][k] > 0 {
                    letters += 1;
                    count += row_map[i][k];
                    letter = k;
                }
            }
            if letters == 1 && count > 1 {
                updated = true;
                rows_to_remove.insert((i, letter));
            }
        }

        let mut cols_to_remove = HashSet::new();
        for &j in cols.iter() {
            let mut letters = 0;
            let mut count = 0usize;
            let mut letter = 0usize;
            for k in 0..26 {
                if col_map[j][k] > 0 {
                    letters += 1;
                    count += col_map[j][k];
                    letter = k;
                }
            }
            if letters == 1 && count > 1 {
                updated = true;
                cols_to_remove.insert((j, letter));
            }
        }

        for &(i, k) in rows_to_remove.iter() {
            rows.remove(&i);
            for &j in cols.iter() {
                col_map[j][k] -= 1;
            }
        }
        for &(j, k) in cols_to_remove.iter() {
            cols.remove(&j);
            for &i in rows.iter() {
                row_map[i][k] -= 1;
            }
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
