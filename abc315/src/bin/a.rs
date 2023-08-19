use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for &c in &s {
        if c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o' {
            continue;
        }
        print!("{}", c);
    }
}
