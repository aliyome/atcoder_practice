use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    };

    let mut s = (0, 0);

    let mut person = vec![];

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                s = (i, j);
            } else if c[i][j] == '>' {
                person.push((i, j, (0, 1)));
            } else if c[i][j] == '<' {
                person.push((i, j, (0, -1)));
            } else if c[i][j] == '^' {
                person.push((i, j, (-1, 0)));
            } else if c[i][j] == 'v' {
                person.push((i, j, (1, 0)));
            }
        }
    }

    // 人物の視界をチェックして通行不可にする
    for &(pi, pj, (vi, vj)) in &person {
        for i in 1.. {
            let ni = pi as i32 + vi * i;
            let nj = pj as i32 + vj * i;

            if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                break;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if c[ni][nj] == '.' || c[ni][nj] == '?' {
                c[ni][nj] = '?';
            } else {
                break;
            }
        }
    }

    // // デバッグ
    // for i in 0..h {
    //     println!("{:?}", c[i]);
    // }

    // 幅優先探索でゴールまでの最短距離を求める
    let mut q = VecDeque::new();
    let mut dist = vec![vec![std::usize::MAX; w]; h];
    dist[s.0][s.1] = 0;
    q.push_back((s.0, s.1, 0));
    while let Some((i, j, d)) = q.pop_front() {
        if dist[i][j] < d {
            continue;
        }
        for &(vi, vj) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ni = i as i32 + vi;
            let nj = j as i32 + vj;

            if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if c[ni][nj] != '.' && c[ni][nj] != 'G' {
                continue;
            }

            if c[ni][nj] == 'G' {
                println!("{}", d + 1);
                return;
            }

            if dist[ni][nj] == std::usize::MAX {
                dist[ni][nj] = d + 1;
                q.push_back((ni, nj, d + 1));
            }
        }
    }

    println!("-1");
}
