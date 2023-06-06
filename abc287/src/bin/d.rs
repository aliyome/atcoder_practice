use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    };

    // 前と後ろの文字列をそれぞれキューで管理する
    let mut front: VecDeque<char> = VecDeque::new();
    let mut back: VecDeque<char> = VecDeque::new();

    // 答えの文字列も同じサイズに分けて管理する
    let mut t_front: VecDeque<char> = VecDeque::new();
    let mut t_back: VecDeque<char> = VecDeque::new();

    // 初期状態はすべて後ろの文字列に入れる
    back.extend(s[s.len() - t.len()..].iter());
    t_back.extend(t.iter());

    // 答えと比較する
    let is_ok = |x: &VecDeque<char>, y: &VecDeque<char>| {
        for i in 0..x.len() {
            // 一致しない文字がある場合はfalse, ?は一致するのでOK
            if x[i] != y[i] && x[i] != '?' && y[i] != '?' {
                return false;
            }
        }
        true
    };

    // 初期状態チェック
    if is_ok(&back, &t_back) {
        println!("Yes");
    } else {
        println!("No");
    }

    // O(N^2) -> TLE
    // 分割位置をずらしながらチェック
    // 一部工夫したけど、やっぱりTLE
    let mut last_front_ok = true;
    for x in 0..t.len() {
        // 前の文字列に追加
        front.push_back(s[x]);
        // 後ろの文字列から削除
        back.pop_front();

        // 答えの文字列も同様
        let p = t_back.pop_front().unwrap();
        t_front.push_back(p);

        // 前の文字列は追加・削除された文字だけチェックする
        if !last_front_ok {
            println!("No");
            continue;
        }
        if s[x] == p || s[x] == '?' || p == '?' {
            last_front_ok = true;
        } else {
            last_front_ok = false;
        }
        // 後ろの文字列はすべてチェックする
        if last_front_ok && is_ok(&back, &t_back) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
