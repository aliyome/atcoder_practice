use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    // 文字種類ごとの最大の長さを求める
    let mut map: HashMap<char, usize> = HashMap::new();

    let mut c = ' ';
    let mut cnt = 1;
    for i in 0..n {
        if c == s[i] {
            cnt += 1;
            if map.get(&c).unwrap_or(&0) < &cnt {
                map.insert(c, cnt);
            }
        } else {
            c = s[i];
            cnt = 1;
            if map.get(&c).unwrap_or(&0) < &cnt {
                map.insert(c, cnt);
            }
        }
    }

    let mut ans = 0;
    for (_, &c) in map.iter() {
        ans += c;
    }

    println!("{}", ans);
}
