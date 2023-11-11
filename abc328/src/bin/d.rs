use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = Vec::new();

    for &c in &s {
        match c {
            'C' => {
                if stack.ends_with(&['A', 'B']) {
                    stack.pop();
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
            _ => stack.push(c),
        }
    }

    let result: String = stack.into_iter().collect();
    println!("{}", result);
}
