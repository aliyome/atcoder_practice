use proconio::{input, marker::Chars};

fn main() {
    input! {
      h: usize,
      w: usize,
      c: [Chars; h]
    }

    let mut dp = vec![vec![0usize; w + 1]; h + 1];
    dp[1][1] = 1;
    for x in 1..=h {
        for y in 1..=w {
            if x == 1 && y == 1 {
                continue;
            }
            let ix = x - 1;
            let iy = y - 1;
            if c[ix][iy] == '#' {
                continue;
            }
            dp[x][y] = dp[x - 1][y] + dp[x][y - 1];
        }
    }

    println!("{}", dp[h][w]);
}
