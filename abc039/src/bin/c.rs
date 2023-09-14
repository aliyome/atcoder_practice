use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let doremi = "WBWBWWBWBWBW".chars().collect_vec();
    let fa = "WBWBWBW".chars().collect_vec();

    let x = [
        "Fa", "Fa", "So", "So", "La", "La", "Si", "Do", "Do", "Re", "Re", "Mi",
    ];

    for i in 0..s.len() - fa.len() {
        if s[i..i + fa.len()] == fa {
            println!("{}", x[(12 - i) % 12]);
            return;
        }
    }
}
