use proconio::input;

fn main() {
    input! {
      (h, w, n): (usize, usize, usize),
      abcd: [(usize, usize, usize, usize); n]
    }

    let mut map = vec![vec![0isize; w + 2]; h + 2];

    // いもす法
    for i in 0..n {
        let (a, b, c, d) = abcd[i];
        map[a][b] += 1;
        map[a][d + 1] -= 1;
        map[c + 1][b] -= 1;
        map[c + 1][d + 1] += 1;
    }
    // h 方向
    for i in 1..=h {
        for j in 1..=w {
            map[i][j] += map[i - 1][j];
        }
    }
    // w 方向
    for i in 1..=h {
        for j in 1..=w {
            map[i][j] += map[i][j - 1];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            if j < w {
                print!("{} ", map[i][j]);
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }
}
