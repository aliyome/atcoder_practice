use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    s.push('\0'); // 番兵 (sentinel

    let mut count = 0;
    let mut total = 0;
    let mut prev_char = '\0'; // 初期値として無効な文字を設定
    let mut map = HashMap::new();

    for c in s {
        if c == prev_char {
            // 前の文字と同じ場合、カウンターを増やす
            count += 1;
        } else {
            // 異なる文字が現れたら、これまでのカウントに基づいて部分文字列の数を計算
            if prev_char != '\0' {
                let current = *map.get(&prev_char).unwrap_or(&0);
                map.insert(prev_char, current.max(count));
            }
            prev_char = c;
            count = 1; // 新しい文字のカウントを開始
        }
    }

    println!("{}", map.values().sum::<usize>());
}
