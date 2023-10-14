use proconio::input;
use std::collections::HashSet;

fn main() {
    // 入力を受け取る
    input! {
        n: usize,
        s: String,
    }

    // まず10^7までの平方数をHashSetに格納する
    let mut squares = HashSet::new();
    for i in 1..=(10_usize.pow(7)) {
        squares.insert(i * i);
    }

    // 文字列を数値配列に変換する
    let digits: Vec<usize> = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // 組合せが重複しないように順列を生成する
    let perm = gen_perm(&digits);
    eprintln!("{}", perm.len());

    let mut count = 0usize;
    for p in &perm {
        let num = p.iter().fold(0, |acc, &x| acc * 10 + x);
        // eprintln!("{}", num);
        if squares.contains(&num) {
            count += 1;
        }
    }

    // 結果を出力する
    println!("{}", count);
}

fn gen_perm(digits: &Vec<usize>) -> Vec<Vec<usize>> {
    if digits.len() == 0 {
        return vec![vec![]];
    }

    let mut used = HashSet::new();
    let mut results = Vec::new();

    for (i, &digit) in digits.iter().enumerate() {
        // 既に使用された数字はスキップする
        if used.contains(&digit) {
            continue;
        }
        used.insert(digit);

        let mut remaining_digits = digits.clone();
        remaining_digits.remove(i);

        let sub_perms = gen_perm(&remaining_digits);
        for mut sub_perm in sub_perms {
            sub_perm.insert(0, digit);
            results.push(sub_perm);
        }
    }

    results
}
