use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    // 5文字以上の場合はありえない
    if s.iter().filter(|&c| *c == 'o').count() > 4 {
        println!("0");
        return;
    }

    // 必ず使う数字
    let mut must = HashSet::new();
    // 使える数字
    let mut digits = vec![];
    for i in 0..10 {
        if s[i] == 'x' {
            continue;
        }
        digits.push(i);
        if s[i] == 'o' {
            must.insert(i);
        }
    }

    // 全探索 10^4
    let mut ans = 0;
    // println!("{:?}", digits);
    // println!("{:?}", must);
    for d1 in &digits {
        for d2 in &digits {
            for d3 in &digits {
                for d4 in &digits {
                    let mut must_used = must.clone();
                    must_used.remove(d1);
                    must_used.remove(d2);
                    must_used.remove(d3);
                    must_used.remove(d4);
                    if must_used.len() == 0 {
                        ans += 1;
                        // println!("{}{}{}{}", d1, d2, d3, d4);
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
