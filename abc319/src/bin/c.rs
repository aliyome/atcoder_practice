use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn disappointed(perm: &Vec<usize>, c: &Vec<Vec<usize>>) -> bool {
    // 各ラインに含まれる数字の数を数える
    let mut rows = vec![HashMap::new(); 3];
    let mut cols = vec![HashMap::new(); 3];
    let mut diag1 = HashMap::new();
    let mut diag2 = HashMap::new();

    // 同じ数字が2つあるかどうか
    let mut rows_ready = vec![false; 3];
    let mut cols_ready = vec![false; 3];
    let mut diag1_ready = false;
    let mut diag2_ready = false;

    // 一つずつ取り出す
    for &pos in perm {
        let i = pos / 3;
        let j = pos % 3;
        let val = c[i][j];

        // 各ラインに含まれる数字の数を数える
        *rows[i].entry(val).or_insert(0) += 1;
        *cols[j].entry(val).or_insert(0) += 1;
        if i == j {
            *diag1.entry(val).or_insert(0) += 1;
        }
        if i + j == 2 {
            *diag2.entry(val).or_insert(0) += 1;
        }

        // 同じ数字が2つある状態で3つ目が出たらがっかり
        if rows_ready[i] {
            return true;
        }
        if cols_ready[j] {
            return true;
        }
        if diag1_ready && i == j {
            return true;
        }
        if diag2_ready && i + j == 2 {
            return true;
        }

        // 同じ数字が2つあるか
        if rows[i].values().any(|&count| count == 2) {
            rows_ready[i] = true;
        }
        if cols[j].values().any(|&count| count == 2) {
            cols_ready[j] = true;
        }
        if diag1.values().any(|&count| count == 2) {
            diag1_ready = true;
        }
        if diag2.values().any(|&count| count == 2) {
            diag2_ready = true;
        }
    }
    false
}

fn main() {
    input! {
        c: [[usize; 3]; 3],
    }

    let perm_list = (0..=8).permutations(9).collect_vec();
    let mut cnt = 0;

    for perm in &perm_list {
        if !disappointed(perm, &c) {
            cnt += 1;
        }
    }

    let probability = cnt as f64 / perm_list.len() as f64;
    println!("{:.27}", probability);
}
