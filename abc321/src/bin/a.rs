use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for i in 0..s.len() - 1 {
        if s[i] <= s[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
