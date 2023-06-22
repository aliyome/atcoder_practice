use proconio::{input, marker::Chars};

fn main() {
    input! {
        o: Chars,
        e: Chars
    };
    for i in 0..o.len().max(e.len()) {
        if i < o.len() {
            print!("{}", o[i]);
        }
        if i < e.len() {
            print!("{}", e[i]);
        }
    }
}
