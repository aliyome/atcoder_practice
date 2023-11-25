use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        grid: [Chars; n],
    }

    // 各行と各列の'o'の数をカウントする
    let mut row_counts = vec![0; n];
    let mut col_counts = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'o' {
                row_counts[i] += 1;
                col_counts[j] += 1;
            }
        }
    }

    // 条件を満たすトリプルの数をカウントする
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'o' {
                count += (row_counts[i] - 1) * (col_counts[j] - 1);
            }
        }
    }

    println!("{}", count);
}
