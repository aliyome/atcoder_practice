use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = VecDeque::new();

    ans.push_back(n);
    for i in (0..n).rev() {
        if s[i] == 'L' {
            ans.push_back(i);
        } else {
            ans.push_front(i);
        }
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
