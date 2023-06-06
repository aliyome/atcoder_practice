use proconio::input;

fn main() {
    input! {
      n: usize, // <= 10^5
      xy: [(usize, usize); n], // x,y <= 1500
      q: usize, // <= 10^5
      abcd:[(usize, usize, usize, usize); q]
    }

    let mut grid = vec![vec![0isize; 1502]; 1502];
    for (x, y) in xy {
        grid[x][y] += 1;
    }

    // 2次元累積和
    // x 方向
    for x in 1..=1500 {
        for y in 1..=1500 {
            grid[x][y] += grid[x - 1][y];
        }
    }
    // y 方向
    for x in 1..=1500 {
        for y in 1..=1500 {
            grid[x][y] += grid[x][y - 1];
        }
    }

    // クエリ処理
    for (a, b, c, d) in abcd {
        let ans = grid[a - 1][b - 1] + grid[c][d] - grid[a - 1][d] - grid[c][b - 1];
        println!("{}", ans);
    }
}
