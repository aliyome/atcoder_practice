use std::{collections::HashMap, io};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s
        .trim_end()
        .split(" ")
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    input! {
        n: usize,
        t: [Chars; n]
    };

    // map[len] = [s1, s2, ...]
    let mut map = HashMap::new();
    for str in t {
        map.entry(str.len()).or_insert(vec![]).push(str);
    }

    let is_ng = |word: &Vec<char>, ng_list: &Vec<Vec<char>>| -> bool {
        for ng in ng_list {
            let mut same = true;
            for i in 0..ng.len() {
                if ng[i] != '*' && ng[i] != word[i] {
                    same = false;
                    break;
                }
            }
            if same {
                return true;
            }
        }
        false
    };

    // 各単語に対して
    for word in &s {
        let len = word.len();

        match map.get(&len) {
            // その単語と同じ長さのNGワードがない場合
            None => {
                // そのまま出力
                print!("{} ", word.iter().collect::<String>());
            }
            // その単語と同じ長さのNGワードがある場合
            Some(ng_list) => {
                // NGワードの中で、その単語と一致するものがあるか
                if is_ng(word, ng_list) {
                    print!("{} ", "*".repeat(len));
                } else {
                    print!("{} ", word.iter().collect::<String>());
                }
            }
        }
    }
}
