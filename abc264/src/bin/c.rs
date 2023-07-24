use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2]
    };

    if h1 < h2 || w1 < w2 {
        println!("No");
        return;
    }

    // 左上のマスを全探索
    for ia in 0..=h1 - h2 {
        for ja in 0..=w1 - w2 {
            // 発見するまでスキップ
            if a[ia][ja] != b[0][0] {
                continue;
            }

            // 列方向に一致するか確認
            let mut skip_rows = HashSet::new();
            let mut row_count = 0;
            for ib in 0.. {
                if ia + ib >= h1 {
                    break;
                }
                if b[row_count][0] == a[ia + ib][ja] {
                    row_count += 1;
                } else {
                    skip_rows.insert(ib);
                }
                if row_count == h2 {
                    break;
                }
            }

            // 行方向に一致するか確認
            let mut skip_cols = HashSet::new();
            let mut col_count = 0;
            for jb in 0.. {
                if ja + jb >= w1 {
                    break;
                }
                if b[0][col_count] == a[ia][ja + jb] {
                    col_count += 1;
                } else {
                    skip_cols.insert(jb);
                }
                if col_count == w2 {
                    break;
                }
            }

            if row_count != h2 || col_count != w2 {
                continue;
            }

            // スキップした行と列を除いた部分が一致するか確認
            let mut ok = true;
            let mut row_count = 0;
            for ib in 0.. {
                if skip_rows.contains(&ib) {
                    continue;
                }
                let mut col_count = 0;
                for jb in 0.. {
                    if skip_cols.contains(&jb) {
                        continue;
                    }
                    if a[ia + ib][ja + jb] != b[row_count][col_count] {
                        ok = false;
                        break;
                    }
                    col_count += 1;
                    if col_count == w2 {
                        break;
                    }
                }
                row_count += 1;
                if row_count == h2 {
                    break;
                }
                if !ok {
                    break;
                }
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
