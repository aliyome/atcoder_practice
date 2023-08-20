use proconio::{input, marker::Chars};

fn main() {
    input! {
        w: Chars,
    };

    for i in 0..w.len() {
        if w[i] != 'a' && w[i] != 'i' && w[i] != 'u' && w[i] != 'e' && w[i] != 'o' {
            print!("{}", w[i]);
        }
    }
    println!();
}
