use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..n {
        if (s[i - 1] == 'a' && s[i] == 'b') || (s[i - 1] == 'b' && s[i] == 'a') {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
