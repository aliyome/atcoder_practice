use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n]
    };

    // Wでソート
    let mut map = BTreeMap::new();
    for i in 0..n {
        map.entry(w[i]).or_insert(vec![]).push(s[i]);
    }

    // X = W[0] のときの一致件数(すべて大人と判定した場合の一致件数)
    let mut sum = s.iter().filter(|&c| *c == '1').count();

    let mut ans = sum;
    for (_, v) in map.iter() {
        for c in v {
            if *c == '1' {
                sum -= 1;
            } else {
                sum += 1;
            }
        }
        ans = ans.max(sum);
    }

    println!("{}", ans);
}
