use proconio::input;

fn main() {
    input! {
      (h, w): (usize, usize),  // H,W <= 1500
      x: [[usize; w]; h],
      q: usize, // Q <= 10^5
      abcd: [(usize, usize, usize, usize); q]
    }

    // grid を 1-index にする
    let mut grid = vec![vec![0; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            grid[i + 1][j + 1] = x[i][j];
        }
    }

    // O(Q)
    for &(a, b, c, d) in &abcd {
        // 縦横2回の累積和 O(HW)
        let mut sum = grid.clone();
        // h 方向の累積和
        for i in a..=c {
            for j in 1..=w {
                sum[i][j] += sum[i - 1][j];
            }
        }
        // w 方向の累積和
        for j in b..=d {
            for i in 1..=h {
                sum[i][j] += sum[i][j - 1];
            }
        }
        // abcd領域の合計
        let ans = sum[a - 1][b - 1] + sum[c][d] - sum[a - 1][d] - sum[c][b - 1];
        println!("{}", ans);
    }
}
