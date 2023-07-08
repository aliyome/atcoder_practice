use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;
const B: usize = 100;

fn main() {
    input! {
      n: usize,
      q: usize,
      s: Chars,
      abcd: [(usize, usize, usize, usize); q]
    }

    // s を char から usize に変換
    let mut s = s
        .iter()
        .map(|&c| c as usize - 'A' as usize)
        .collect::<Vec<_>>();

    // b の事前計算
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = B.pow(i as u32);
        b[i] %= MOD;
    }

    // hash
    let hash = |s: &[usize]| -> usize {
        let mut h = 0;
        for i in 0..n {
            h += b[i - n - 1] * (s[i] as usize);
            h %= MOD;
        }
        h
    };

    for (a, b, c, d) in abcd {}

    println!("{}", n);
}
