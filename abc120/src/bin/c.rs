use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;
    let mut seq = HashMap::new();
    *seq.entry(s[0]).or_insert(0) += 1;
    for i in 1..s.len() {
        *seq.entry(s[i]).or_insert(0) += 1;
        if (s[i] == '0' && *seq.entry('1').or_default() > 0)
            || (s[i] == '1' && *seq.entry('0').or_default() > 0)
        {
            ans += 2;
            *seq.entry('0').or_default() -= 1;
            *seq.entry('1').or_default() -= 1;
        }
    }
    println!("{}", ans);
}
