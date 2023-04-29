use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut grid = vec![vec!['.'; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            grid[i + 1][j + 1] = c[i][j];
        }
    }
    let n = std::cmp::min(h, w);
    let mut s = vec![0; n + 1];

    for a in 1..=h {
        for b in 1..=w {
            if grid[a][b] != '#' {
                continue;
            }
            for d in 1..=n {
                if grid[a + d][b + d] != '#'
                    || grid[a + d][b - d] != '#'
                    || grid[a - d][b + d] != '#'
                    || grid[a - d][b - d] != '#'
                {
                    break;
                }
                if d < n
                    && grid[a + d + 1][b + d + 1] == '#'
                    && grid[a + d + 1][b - d - 1] == '#'
                    && grid[a - d - 1][b + d + 1] == '#'
                    && grid[a - d - 1][b - d - 1] == '#'
                {
                    continue;
                }
                s[d] += 1;
            }
        }
    }

    for i in 1..=n {
        if i > 1 {
            print!(" ");
        }
        print!("{}", s[i]);
    }
    println!();
}
