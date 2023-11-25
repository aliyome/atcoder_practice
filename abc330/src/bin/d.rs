use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        grid: [Chars; n],
    }

    let mut row_counts = vec![0usize; n];
    let mut col_counts = vec![0usize; n];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'o' {
                row_counts[i] += 1;
                col_counts[j] += 1;
            }
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'o' {
                // 各行と各列で「o」が2つ以上ある場合のみカウント
                if row_counts[i] > 1 && col_counts[j] > 1 {
                    count += (row_counts[i] - 1) * (col_counts[j] - 1);
                }
            }
        }
    }

    println!("{}", count);
}
