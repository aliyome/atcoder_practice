use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }
    // 素朴に実装すると 2000 x 2000 x (2000 + 2000) で間に合わない
    // 列のみの総和と行のみの総和を別々に計算しておく
    let mut cols = vec![0usize; w];
    let mut rows = vec![0usize; h];

    // 列
    for j in 0..w {
        for i in 0..h {
            cols[j] += a[i][j];
        }
    }
    // 行
    for i in 0..h {
        for j in 0..w {
            rows[i] += a[i][j];
        }
    }

    // 出力
    for i in 0..h {
        for j in 0..w {
            print!("{} ", cols[j] + rows[i] - a[i][j]);
        }
        println!();
    }
}
