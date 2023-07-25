use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    };

    let mut b = a.clone();
    for j in 1..n {
        b[0][j] = a[0][j - 1];
    }
    for i in 1..n {
        b[i][n - 1] = a[i - 1][n - 1];
    }
    for j in 0..n - 1 {
        b[n - 1][j] = a[n - 1][j + 1];
    }
    for i in 0..n - 1 {
        b[i][0] = a[i + 1][0];
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
