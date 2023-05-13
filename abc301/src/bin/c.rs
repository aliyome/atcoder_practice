use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_chars = HashMap::new();
    let mut t_chars = HashMap::new();

    let possible_chars: HashSet<char> = ['a', 't', 'c', 'o', 'd', 'e', 'r']
        .iter()
        .cloned()
        .collect();

    for i in 0..s.len() {
        *s_chars.entry(s[i]).or_insert(0) += 1;
        *t_chars.entry(t[i]).or_insert(0) += 1;
    }

    let alphas = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut diff = 0;
    for &c in alphas.iter() {
        let s_cnt = *s_chars.entry(c).or_default();
        let t_cnt = *t_chars.entry(c).or_default();
        let d = (s_cnt as i32 - t_cnt as i32).abs();
        if d > 0 && !possible_chars.contains(&c) {
            println!("No");
            return;
        }
        diff += d;
    }
    let s_joker = *s_chars.entry('@').or_default();
    let t_joker = *t_chars.entry('@').or_default();

    if diff <= s_joker + t_joker {
        println!("Yes");
    } else {
        println!("No");
    }
}
