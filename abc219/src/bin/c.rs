use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n],
    };

    let mut map = HashMap::new();
    for (i, c) in x.iter().enumerate() {
        map.insert(c, i);
    }

    s.sort_by(|a, b| {
        let mut i = 0;
        while i < a.len() && i < b.len() {
            if a[i] != b[i] {
                return map.get(&a[i]).unwrap().cmp(map.get(&b[i]).unwrap());
            }
            i += 1;
        }
        a.len().cmp(&b.len())
    });

    for n in s {
        println!("{}", n.iter().collect::<String>());
    }
}
