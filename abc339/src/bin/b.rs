use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = vec![vec!['.'; w]; h];
    let (mut x, mut y) = (0, 0);
    let mut direction = 0; // 0: 上, 1: 右, 2: 下, 3: 左

    for _ in 0..n {
        if grid[x][y] == '.' {
            grid[x][y] = '#';
            direction = (direction + 1) % 4;
        } else {
            grid[x][y] = '.';
            direction = (direction + 3) % 4;
        }

        match direction {
            0 => x = if x == 0 { h - 1 } else { x - 1 },
            1 => y = (y + 1) % w,
            2 => x = (x + 1) % h,
            3 => y = if y == 0 { w - 1 } else { y - 1 },
            _ => unreachable!(),
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
