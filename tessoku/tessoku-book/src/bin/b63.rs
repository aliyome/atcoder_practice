use proconio::{input, marker::Chars};

fn main() {
    input! {
      r: usize,
      c: usize,
      sy: usize,
      sx: usize,
      gy: usize,
      gx: usize,
      maze: [Chars; r]
    }

    let mut dist = vec![vec![std::usize::MAX; c]; r];
    dist[sy - 1][sx - 1] = 0;

    let mut q = std::collections::VecDeque::new();
    q.push_back((sy - 1, sx - 1, 0));
    while let Some((y, x, d)) = q.pop_front() {
        for &(vy, vx) in [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)].iter() {
            let ny = y as isize + vy;
            let nx = x as isize + vx;
            if ny < 0 || ny >= r as isize || nx < 0 || nx >= c as isize {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if maze[ny][nx] == '#' {
                continue;
            }
            if dist[ny][nx] <= d + 1 {
                continue;
            }
            dist[ny][nx] = d + 1;
            q.push_back((ny, nx, d + 1));
        }
    }

    println!("{}", dist[gy - 1][gx - 1]);
}
