use itertools::Itertools;
use proconio::input;

// TODO: 無駄な処理が多い
fn main() {
    input! {
        n: usize,
    };

    let bit_str = format!("{:b}", n);
    let mut bits = vec![];
    for i in 0..bit_str.len() {
        if bit_str.chars().rev().nth(i).unwrap() == '1' {
            bits.push(1usize << i);
        }
    }

    let mut ans = vec![];
    for i in 1..=bits.len() {
        for vv in bits.iter().combinations(i).collect_vec() {
            let mut sum = 0;
            for v in vv {
                sum += v;
            }
            ans.push(sum);
        }
    }

    ans.sort();
    println!("0");
    for a in ans {
        println!("{}", a);
    }
}
