use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let s = s;
    let mut t = s.clone();
    t.reverse();

    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("No");
            return;
        }
    }
    if s[0..s.len() / 2].iter().collect::<Vec<_>>()
        != s[s.len() / 2 + 1..].iter().collect::<Vec<_>>()
    {
        println!("No");
        return;
    }

    println!("Yes");
}
