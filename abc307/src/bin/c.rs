use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        ha: usize,
        wa: usize,
        a: [Chars; ha],
        hb: usize,
        wb: usize,
        b: [Chars; hb],
        hx: usize,
        wx: usize,
        x: [Chars; hx]
    };

    // 二次元配列を集合に変換する
    let conv_to_set = |grid: &Vec<Vec<char>>| -> HashSet<(usize, usize)> {
        let mut set = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '#' {
                    set.insert((i, j));
                }
            }
        }
        set
    };
    let a_set = conv_to_set(&a);
    let b_set = conv_to_set(&b);
    let x_set = conv_to_set(&x);

    // 移動したパターンを生成する
    let move_sheet =
        |grid: &HashSet<(usize, usize)>, i: isize, j: isize| -> HashSet<(usize, usize)> {
            let mut moved = HashSet::new();
            for (y, x) in grid {
                let y = *y as isize + i;
                let x = *x as isize + j;
                if y < 0 || x < 0 {
                    return HashSet::new();
                }
                moved.insert((y as usize, x as usize));
            }
            moved
        };
    let mut a_moved = vec![];
    for i in -(ha as isize)..=ha as isize {
        for j in -(wa as isize)..=wa as isize {
            a_moved.push(move_sheet(&a_set, i, j));
        }
    }
    let mut b_moved = vec![];
    for i in -(hb as isize)..=hb as isize {
        for j in -(wb as isize)..=wb as isize {
            b_moved.push(move_sheet(&b_set, i, j));
        }
    }

    // 移動したパターンを重ねる
    for a in a_moved.iter().filter(|a| !a.is_empty()) {
        for b in b_moved.iter().filter(|b| !b.is_empty()) {
            let c = a.union(b).copied().collect::<HashSet<_>>();
            if c == x_set {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
