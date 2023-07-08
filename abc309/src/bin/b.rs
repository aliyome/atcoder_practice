use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    };

    let mut b = a.clone();

    // 1行目
    for j in 1..n {
        b[0][j] = a[0][j - 1];
    }
    // n-1列目
    for i in 1..n {
        b[i][n - 1] = a[i - 1][n - 1];
    }
    // n-1行目
    for j in (0..n - 1).rev() {
        b[n - 1][j] = a[n - 1][j + 1];
    }
    // 0列目
    for i in (0..n - 1).rev() {
        b[i][0] = a[i + 1][0];
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!();
        // println!("{:?}", b[i]);
    }
}
