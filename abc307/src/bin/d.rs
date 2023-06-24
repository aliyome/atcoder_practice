use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut stack = vec![Vec::new()];
    for c in s {
        match c {
            '(' => {
                stack.push(Vec::new());
                let n = stack.len();
                stack[n - 1].push(c);
            }
            ')' => {
                let n = stack.len();
                if stack.len() == 1 {
                    stack[n - 1].push(c);
                } else {
                    stack.pop();
                }
            }
            _ => {
                let n = stack.len();
                stack[n - 1].push(c);
            }
        }
    }

    let ans: String = stack.into_iter().flatten().collect();
    println!("{}", ans);
}
