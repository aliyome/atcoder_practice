use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
      h: usize,
      w: usize,
      k: usize,
      c: [Chars; h]
    }

    let mut ans = 0;
    for i in 0..(1 << h) as usize {
        let mut rows = vec![];
        for j in 0..h {
            if i & (1 << j) != 0 {
                rows.push(j);
            }
        }

        if rows.len() > k {
            continue;
        }

        let mut c = c.clone();
        for &r in &rows {
            for j in 0..w {
                c[r][j] = '#';
            }
        }

        // 列ごとの白マスの数
        let mut white_count = BinaryHeap::new();
        for j in 0..w {
            let mut sum = 0;
            for i in 0..h {
                if c[i][j] == '.' {
                    sum += 1;
                }
            }
            white_count.push((sum, j));
        }

        // 白マスが多い順に残りの列を選ぶ
        for _ in 0..k - rows.len() {
            let (_, j) = white_count.pop().unwrap();
            for i in 0..h {
                if c[i][j] == '.' {
                    c[i][j] = '#';
                }
            }
        }

        // 数を数える
        let mut total = 0;
        for i in 0..h {
            for j in 0..w {
                if c[i][j] == '#' {
                    total += 1;
                }
            }
        }
        ans = ans.max(total);
    }

    println!("{}", ans);
}
