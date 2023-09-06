use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        p: [[usize; w]; h]
    }

    let mut ans = 0;
    for bit in 1..1 << h {
        let mut rows = vec![];
        for i in 0..h {
            if bit & 1 << i == 0 {
                continue;
            }
            rows.push(i);
        }

        let mut map = HashMap::new();
        for j in 0..w {
            let mut col_map = HashMap::new();
            for &i in &rows {
                *col_map.entry(p[i][j]).or_insert(0) += 1;
            }
            if col_map.keys().len() > 1 {
                continue;
            }
            *map.entry(p[rows[0]][j]).or_insert(0) += 1;
        }

        let mut max_cols = 0;
        for (_, v) in map.iter() {
            max_cols = max_cols.max(*v);
        }

        ans = ans.max(rows.len() * max_cols);
    }

    println!("{}", ans);
}
