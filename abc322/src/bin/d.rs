use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        p: [[Chars; 4]; 3],
    }

    // ポリオミノが合計16マスじゃなかったら敷き詰められない
    if p.iter().flatten().flatten().filter(|&&c| c == '#').count() != 16 {
        println!("No");
        return;
    }

    // ポリオミノ
    let mut minos = vec![];

    // 範囲外エラーを対処しないでいいように集合に変換する
    for p in &p {
        let mut set = HashSet::new();
        for i in 0..4 {
            for j in 0..4 {
                if p[i][j] == '#' {
                    set.insert((i as isize, j as isize));
                }
            }
        }
        // 0 度回転したポリオミノ
        minos.push(vec![set]);
    }

    // それぞれのポリオミノが 0, 90, 180, 270度回転したポリオミノを作る
    for i in 0..3 {
        let mut mino = minos[i][0].clone();
        for _ in 1..=3 {
            mino = rotate(&mino);
            minos[i].push(mino.clone());
        }
    }

    // それぞれのポリオミノの座標を左上に隙間がなくなるように正規化する
    let mut normalized_minos = vec![];
    for list in &minos {
        let mut normalized_list = vec![];
        for mino in list {
            let mut normalized = HashSet::new();
            let mut min_i = 10;
            let mut min_j = 10;
            for (i, j) in mino.iter() {
                min_i = min_i.min(*i);
                min_j = min_j.min(*j);
            }
            for (i, j) in mino.iter() {
                normalized.insert((*i - min_i, *j - min_j));
            }
            normalized_list.push(normalized);
        }
        normalized_minos.push(normalized_list);
    }

    // 全組み合わせを試す
    for mino1 in &normalized_minos[0] {
        for mino2 in &normalized_minos[1] {
            for mino3 in &normalized_minos[2] {
                // 全位置を試す
                for di1 in 0..4 {
                    for dj1 in 0..4 {
                        for di2 in 0..4 {
                            for dj2 in 0..4 {
                                for di3 in 0..4 {
                                    for dj3 in 0..4 {
                                        // 3つのポリオミノを敷き詰める
                                        let mut set = HashSet::new();
                                        for &(i, j) in mino1 {
                                            set.insert((i + di1, j + dj1));
                                        }
                                        for &(i, j) in mino2 {
                                            set.insert((i + di2, j + dj2));
                                        }
                                        for &(i, j) in mino3 {
                                            set.insert((i + di3, j + dj3));
                                        }

                                        // 範囲外にミノが出たらダメ
                                        let mut out_of_range = false;
                                        for &(i, j) in &set {
                                            if i < 0 || i > 3 || j < 0 || j > 3 {
                                                out_of_range = true;
                                                break;
                                            }
                                        }
                                        if out_of_range {
                                            continue;
                                        }

                                        // 敷き詰められたら Yes を出力して終了
                                        if set.len() == 16 {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}

// 90 度回転
fn rotate(mino: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut new_mino = HashSet::new();
    for (i, j) in mino {
        new_mino.insert((*j, -*i));
    }
    new_mino
}
