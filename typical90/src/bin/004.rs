use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    // 各行, 各列の合計を計算
    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];
    for i in 0..w {
        for j in 0..h {
            row_sum[j] += a[j][i];
            col_sum[i] += a[j][i];
        }
    }

    // 各行・各列の値を合計して出力する。ただし元の値は行・列どちらにも含まれているので、もとの値を引く
    for i in 0..h {
        for j in 0..w {
            print!("{} ", row_sum[i] + col_sum[j] - a[i][j]);
        }
        println!();
    }
}
