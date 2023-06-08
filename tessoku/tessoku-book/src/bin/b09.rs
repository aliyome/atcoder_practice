use proconio::input;

fn main() {
    input! {
      n: usize,
      abcd: [(usize, usize, usize, usize); n]
    }

    let mut map = vec![vec![0; 1502]; 1502];
    for (a, b, c, d) in abcd {
        map[a][b] += 1;
        map[c][d] += 1;
        map[a][d] -= 1;
        map[c][b] -= 1;
    }
    // H方向
    for i in 1..=1500 {
        for j in 1..=1500 {
            map[i][j] += map[i - 1][j];
        }
    }
    // W方向
    for i in 1..=1500 {
        for j in 1..=1500 {
            map[i][j] += map[i][j - 1];
        }
    }

    // 面積のカウント
    let mut cnt = 0;
    for i in 1..=1500 {
        for j in 1..=1500 {
            if map[i][j] > 0 {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
