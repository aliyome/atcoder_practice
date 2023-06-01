use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    // input! {
    //     x: usize,
    // };
    // // 愚直に実装 CE
    // let mut ans = 0;
    // for k in 0..10usize.pow(100) {
    //     x /= 10usize.pow(k);
    // }
    // println!("{}", ans);

    input! {
        x: Chars
    }

    // 1桁ずつの数字に変換
    let x: Vec<usize> = x.into_iter().map(|c| c as usize - '0' as usize).collect();

    // 各桁までの和を計算
    let mut sum = vec![0];
    for &x in &x {
        sum.push(sum.last().unwrap() + x);
    }

    // 末尾から数字を決定していくため逆順にする
    sum.reverse();

    let mut ans = vec![];
    let mut c = 0;
    for i in 0..sum.len() {
        let a = sum[i] + c;
        let b = a % 10;
        ans.push(b);
        c = a / 10;
    }

    while c > 0 {
        ans.push(c % 10);
        c /= 10;
    }

    let ans = ans
        .iter()
        .rev()
        .map(|&d| format!("{}", d))
        .collect::<String>();

    println!("{}", ans.trim_start_matches('0'));
}
