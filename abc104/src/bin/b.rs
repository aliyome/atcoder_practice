use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    if s[0] != 'A' {
        println!("WA");
        return;
    }

    if s[2..s.len() - 1].iter().filter(|&c| *c == 'C').count() != 1 {
        println!("WA");
        return;
    }

    if s[2..s.len() - 1]
        .iter()
        .filter(|&c| *c != 'C')
        .any(|c| c.is_uppercase())
    {
        println!("WA");
        return;
    }

    if s[s.len() - 1].is_uppercase() {
        println!("WA");
        return;
    }

    println!("AC");
}
