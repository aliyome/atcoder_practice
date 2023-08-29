use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    };

    for j in 0..w + 2 {
        print!("#");
    }
    println!();
    for i in 0..h {
        print!("#");
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        print!("#");
        println!();
    }
    for j in 0..w + 2 {
        print!("#");
    }
    println!();
}
