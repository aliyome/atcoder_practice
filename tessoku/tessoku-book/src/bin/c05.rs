use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let mut lucky_list = BTreeSet::new();
    for i in 0..1 << 10 {
        let mut lucky = vec![];
        for j in 0..10 {
            if i & 1 << j != 0 {
                lucky.push(7);
            } else {
                lucky.push(4);
            }
        }
        let str = lucky
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join("");
        lucky_list.insert(str);
    }

    println!("{}", lucky_list.iter().skip(n - 1).next().unwrap());
}
