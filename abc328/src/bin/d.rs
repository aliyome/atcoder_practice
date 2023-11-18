use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = vec![vec![]];
    for c in s {
        match c {
            'A' => stack.push(vec![c]),
            'B' => stack.last_mut().unwrap().push(c),
            'C' => {
                stack.last_mut().unwrap().push(c);
                if stack.last().unwrap() == &['A', 'B', 'C'] {
                    stack.pop();
                }
            }
            _ => unreachable!(),
        }
    }

    let ans = stack.iter().flatten().collect::<String>();
    if ans != "" {
        println!("{}", ans);
    }

    // eprintln!("{:?}", stack);
}
