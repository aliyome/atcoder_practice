use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // 全探索で解く
    let mut ans: Vec<char> = vec![];
    let mut idx = 0;
    for kk in 0..k {
        for c in 0..26 {
            for nn in idx..n - k + kk {
                let c = (c as u8 + b'a') as char;
                if s[nn] == c {
                    ans.push(c);
                    idx = nn;
                    break;
                }
            }
            if ans.len() > kk {
                break;
            }
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
