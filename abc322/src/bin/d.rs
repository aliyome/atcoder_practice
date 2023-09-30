use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        polyominoes: [Chars; 12],
    }

    // 各ポリオミノの座標をHashSet<(isize, isize)>で表現し、正規化します。
    let mut shapes = Vec::new();
    for p in 0..3 {
        let mut shape = HashSet::new();
        let mut min_row = 4;
        let mut min_col = 4;
        for i in 0..4 {
            for j in 0..4 {
                if polyominoes[i + 4 * p][j] == '#' {
                    min_row = min_row.min(i);
                    min_col = min_col.min(j);
                    shape.insert((i as isize, j as isize));
                }
            }
        }
        let normalized_shape: HashSet<(isize, isize)> = shape
            .iter()
            .map(|&(r, c)| (r - min_row as isize, c - min_col as isize))
            .collect();
        shapes.push(normalized_shape);
    }

    // すべてのポリオミノの配置と回転を試します。
    for &rotation1 in &[0, 1, 2, 3] {
        for &rotation2 in &[0, 1, 2, 3] {
            for &rotation3 in &[0, 1, 2, 3] {
                for row1 in 0..4 {
                    for col1 in 0..4 {
                        for row2 in 0..4 {
                            for col2 in 0..4 {
                                for row3 in 0..4 {
                                    for col3 in 0..4 {
                                        let mut grid = HashSet::new();
                                        let mut valid = true;

                                        // ポリオミノ1を配置します。
                                        for &(r, c) in
                                            &rotate_and_translate(&shapes[0], rotation1, row1, col1)
                                        {
                                            if !grid.insert((r, c))
                                                || r < 0
                                                || r >= 4
                                                || c < 0
                                                || c >= 4
                                            {
                                                valid = false;
                                                break;
                                            }
                                        }

                                        // ポリオミノ2を配置します。
                                        for &(r, c) in
                                            &rotate_and_translate(&shapes[1], rotation2, row2, col2)
                                        {
                                            if !grid.insert((r, c))
                                                || r < 0
                                                || r >= 4
                                                || c < 0
                                                || c >= 4
                                            {
                                                valid = false;
                                                break;
                                            }
                                        }

                                        // ポリオミノ3を配置します。
                                        for &(r, c) in
                                            &rotate_and_translate(&shapes[2], rotation3, row3, col3)
                                        {
                                            if !grid.insert((r, c))
                                                || r < 0
                                                || r >= 4
                                                || c < 0
                                                || c >= 4
                                            {
                                                valid = false;
                                                break;
                                            }
                                        }

                                        // グリッドが全て埋まっているか確認します。
                                        if valid && grid.len() == 16 {
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
fn rotate_and_translate(
    shape: &HashSet<(isize, isize)>,
    rotation: usize,
    row: isize,
    col: isize,
) -> HashSet<(isize, isize)> {
    let mut rotated = HashSet::new();
    for &(r, c) in shape {
        let (nr, nc) = match rotation {
            1 => (c, 3 - r),
            2 => (3 - r, 3 - c),
            3 => (3 - c, r),
            _ => (r, c),
        };
        rotated.insert((nr + row, nc + col));
    }
    rotated
}
