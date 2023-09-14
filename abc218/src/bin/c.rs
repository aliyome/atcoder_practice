use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n]
    };

    let mut s_set = BTreeSet::new();
    let mut t_set = BTreeSet::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                s_set.insert((i as isize, j as isize));
            }
            if t[i][j] == '#' {
                t_set.insert((i as isize, j as isize));
            }
        }
    }

    // 90度回転 x 3
    for _ in 0..4 {
        // 平行移動
        let (si, sj) = *s_set.first().unwrap();
        let (ti, tj) = *t_set.first().unwrap();
        let (di, dj) = (ti - si, tj - sj);

        let mut s = BTreeSet::new();
        for (i, j) in s_set.iter() {
            s.insert((i + di, j + dj));
        }
        if s == t_set {
            println!("Yes");
            return;
        }
        // 回転
        let mut rotated = BTreeSet::new();
        for (i, j) in s_set.iter() {
            rotated.insert((*j, n as isize - i - 1));
        }
        s_set = rotated;
    }
    println!("No");
}
