use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        polyominoes: [Chars; 12],
    }

    // ポリオミノが合計16マスじゃなかったら敷き詰められない
    let mut count = 0;
    for i in 0..12 {
        for j in 0..4 {
            if polyominoes[i][j] == '#' {
                count += 1;
            }
        }
    }
    if count != 16 {
        println!("No");
        return;
    }

    // 各ポリオミノの座標をHashSet<(isize, isize)>で表現し、正規化します。
    let mut shapes = Vec::new();
    for p in 0..3 {
        let mut shape = HashSet::new();
        for i in 0..4 {
            for j in 0..4 {
                if polyominoes[i + 4 * p][j] == '#' {
                    shape.insert((i as isize, j as isize));
                }
            }
        }
        shapes.push(shape);
    }

    // 最終的な形
    let reference = (0..4)
        .flat_map(|i| (0..4).map(move |j| (i as isize, j as isize)))
        .collect::<HashSet<_>>();

    // すべてのポリオミノの配置と回転を試します。
    for &rotation1 in &[0, 1, 2, 3] {
        let shape1 = normalize(&shapes[0], rotation1);
        for &rotation2 in &[0, 1, 2, 3] {
            let shape2 = normalize(&shapes[1], rotation2);
            for &rotation3 in &[0, 1, 2, 3] {
                let shape3 = normalize(&shapes[2], rotation3);
                for row1 in 0..4 {
                    for col1 in 0..4 {
                        for row2 in 0..4 {
                            for col2 in 0..4 {
                                for row3 in 0..4 {
                                    for col3 in 0..4 {
                                        let mut grid = HashSet::new();
                                        let shape1 = translate(&shape1, row1, col1);
                                        grid.extend(shape1);

                                        let shape2 = translate(&shape2, row2, col2);
                                        grid.extend(shape2);

                                        let shape3 = translate(&shape3, row3, col3);
                                        grid.extend(shape3);

                                        if grid == reference {
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

    // 全ての配置を試した結果、条件を満たす配置がなかった場合
    println!("No");
}

// 指定された回転と移動を適用してポリオミノの座標を計算します。
fn translate(shape: &HashSet<(isize, isize)>, row: isize, col: isize) -> HashSet<(isize, isize)> {
    let mut ret = HashSet::new();
    for &(r, c) in shape {
        ret.insert((r + row, c + col));
    }
    ret
}

fn rotation(shape: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut rotated = HashSet::new();
    for &(r, c) in shape {
        rotated.insert((c, -r));
    }
    rotated
}

fn n_rotation(shape: &HashSet<(isize, isize)>, n: usize) -> HashSet<(isize, isize)> {
    let mut rotated = shape.clone();
    for _ in 0..n {
        rotated = rotation(&rotated);
    }
    rotated
}

fn normalize(shape: &HashSet<(isize, isize)>, rot: i32) -> HashSet<(isize, isize)> {
    let shape = n_rotation(shape, rot as usize);
    let min_row = shape.iter().map(|&(r, _)| r).min().unwrap_or(0);
    let min_col = shape.iter().map(|&(_, c)| c).min().unwrap_or(0);
    shape
        .iter()
        .map(|&(r, c)| (r - min_row, c - min_col))
        .collect()
}
