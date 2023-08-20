use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    };

    // 愚直実装
    let mut marked = vec![vec![false; w]; h];
    loop {
        let mut updated = false;
        // 各行・列が1種類のクッキーで構成されているかを確認する
        for j in 0..w {
            let mut set = HashSet::new();
            let mut count = 0;
            for i in 0..h {
                if marked[i][j] {
                    continue;
                }
                set.insert(c[i][j]);
                count += 1;
            }
            if set.len() == 1 && count > 1 {
                for i in 0..h {
                    marked[i][j] = true;
                }
                updated = true;
            }
        }
        for i in 0..h {
            let mut set = HashSet::new();
            let mut count = 0;
            for j in 0..w {
                if marked[i][j] {
                    continue;
                }
                set.insert(c[i][j]);
                count += 1;
            }
            if set.len() == 1 && count > 1 {
                for j in 0..w {
                    marked[i][j] = true;
                }
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }

    // 残ったクッキーの数を数える
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if !marked[i][j] {
                ans += 1;
            }
        }
    }

    // // DEBUG
    // for i in 0..h {
    //     for j in 0..w {
    //         if !marked[i][j] {
    //             ans += 1;
    //             print!("{}", c[i][j]);
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    println!("{}", ans);
}
