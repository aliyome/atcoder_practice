use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    for i in 0..h {
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        println!();
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        println!();
    }
}
