use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    for i in 0..h {
        for _ in 0..2 {
            for j in 0..w {
                print!("{}", c[i][j]);
            }
            println!();
        }
    }
}
