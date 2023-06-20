use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize
    };

    let mut dist = vec![vec![10usize.pow(9); n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=n {
            dist[i][j] = (i as isize - j as isize).abs() as usize;
        }
    }

    for i in 1..=n {
        // ショートカットした先からbfs
        let mut base = if dist[i][x] < dist[i][y] {
            dist[i][x] + 1
        } else {
            dist[i][y] + 1
        };

        let mut dist_from_y = 0;
        while dist_from_y < n {
            if y + dist_from_y <= n {
                dist[i][y + dist_from_y] = (base + dist_from_y).min(dist[i][y + dist_from_y]);
            }
            if 1 + dist_from_y <= y {
                dist[i][y - dist_from_y] = (base + dist_from_y).min(dist[i][y - dist_from_y]);
            }
            dist_from_y += 1;
        }
    }

    let mut ans = vec![0; n + 1];
    for i in 1..n {
        for j in i..=n {
            if i == j {
                continue;
            }
            ans[dist[i][j]] += 1;
        }
    }

    for i in 1..n {
        println!("{}", ans[i]);
    }
}
