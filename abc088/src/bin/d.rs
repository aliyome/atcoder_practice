use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    let mut q = VecDeque::new();
    dist[0][0] = 1;
    q.push_back((0isize, 0isize));
    while let Some((i, j)) = q.pop_front() {
        for &(vi, vj) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let (ni, nj) = (i + vi, j + vj);
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if s[ni][nj] == '#' {
                continue;
            }
            if dist[ni][nj] <= dist[i as usize][j as usize] + 1 {
                continue;
            }
            dist[ni][nj] = dist[i as usize][j as usize] + 1;
            q.push_back((ni as isize, nj as isize));
        }
    }

    if dist[h - 1][w - 1] == std::usize::MAX {
        println!("-1");
        return;
    }

    let mut block = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                block += 1;
            }
        }
    }

    let ans = h * w - dist[h - 1][w - 1] - block;
    println!("{}", ans);
}
