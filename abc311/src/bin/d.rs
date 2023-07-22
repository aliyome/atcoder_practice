use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [proconio::marker::Chars; n],
    }

    let dx = [0isize, 1, 0, -1];
    let dy = [1isize, 0, -1, 0];
    let mut stopped = vec![vec![false; m]; n];
    let mut visited = vec![vec![false; m]; n];
    let mut deque = VecDeque::new();
    deque.push_back((1, 1));
    stopped[1][1] = true;
    visited[1][1] = true;

    while let Some((x, y)) = deque.pop_front() {
        for d in 0..4 {
            let mut nx = x;
            let mut ny = y;
            while grid[(nx + dx[d]) as usize][(ny + dy[d]) as usize] == '.' {
                nx += dx[d];
                ny += dy[d];
                visited[nx as usize][ny as usize] = true;
            }
            if !stopped[nx as usize][ny as usize] {
                stopped[nx as usize][ny as usize] = true;
                deque.push_back((nx as isize, ny as isize));
            }
        }
    }

    let mut ans = 0usize;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '#' && visited[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
