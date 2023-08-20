use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        w: usize
    };

    for i in (0..s.len()).step_by(w) {
        print!("{}", s[i]);
    }
}
