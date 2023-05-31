use std::collections::VecDeque;

use proconio::{input, marker::Chars};

// 01-bfs

fn main() {
    input! {
        (h, w): (usize, usize),
        (rs, cs): (usize, usize),
        (rt, ct): (usize, usize),
        g: [Chars; h]
    }

    // grid を 1-indexed にする
    let mut grid = vec![vec!['#'; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            grid[i + 1][j + 1] = g[i][j];
        }
    }

    // dp[i][j][d] = d(0:上, 1:右, 2:下, 3:左)を向いた状態で(i, j)にいるときの折返し回数
    let mut dp = vec![vec![vec![10usize.pow(9); 4]; w + 1]; h + 1];
    dp[rs][cs][0] = 0;
    dp[rs][cs][1] = 0;
    dp[rs][cs][2] = 0;
    dp[rs][cs][3] = 0;

    // 01-bfs
    let mut q = VecDeque::new();
    q.push_back((rs, cs, 0));
    q.push_back((rs, cs, 1));
    q.push_back((rs, cs, 2));
    q.push_back((rs, cs, 3));

    // delta (up, right, down, left)
    let v = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some((r, c, dir)) = q.pop_front() {
        for next_dir in 0..4 {
            let (vr, vc) = v[next_dir];
            let nr = r as i64 + vr;
            let nc = c as i64 + vc;
            // 範囲チェック
            if nr < 0 || nr > h as i64 || nc < 0 || nc > w as i64 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            // 壁チェック
            if grid[nr][nc] == '#' {
                continue;
            }
            // dpに移動コストを代入
            if next_dir == dir {
                // 同じ方向に進む場合はコストを増やさない
                if dp[nr][nc][next_dir] > dp[r][c][dir] {
                    dp[nr][nc][next_dir] = dp[r][c][dir];
                    q.push_front((nr, nc, next_dir));
                }
            } else {
                // 方向が異なる場合はコストを増やす
                if dp[nr][nc][next_dir] > dp[r][c][dir] + 1 {
                    dp[nr][nc][next_dir] = dp[r][c][dir] + 1;
                    q.push_back((nr, nc, next_dir));
                }
            }
        }
    }

    println!(
        "{}",
        dp[rt][ct][0]
            .min(dp[rt][ct][1])
            .min(dp[rt][ct][2])
            .min(dp[rt][ct][3])
    );
}
