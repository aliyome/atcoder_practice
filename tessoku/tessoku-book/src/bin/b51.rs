use proconio::{input, marker::Chars};

fn main() {
    input! {
      s: Chars,
    }

    let mut stack = vec![];
    for i in 0..s.len() {
        if s[i] == '(' {
            stack.push(i);
        } else if s[i] == ')' {
            if let Some(l) = stack.pop() {
                println!("{} {}", l + 1, i + 1);
            }
        }
    }
}
