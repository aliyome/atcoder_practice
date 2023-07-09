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
        .map(|&c| c as usize - 'a' as usize + 1)
        .collect::<Vec<_>>();
    // 1-indexed
    s.insert(0, 0);

    // power100 の事前計算
    let mut power100 = vec![0; n + 1];
    power100[0] = 1;
    for i in 1..=n {
        power100[i] = power100[i - 1] * B;
        power100[i] %= MOD;
    }

    // hash[i] := 文文字列[1,i]のhashの事前計算
    let mut hash = vec![0; n + 1];
    for i in 1..=n {
        hash[i] = B * hash[i - 1] + s[i];
        hash[i] %= MOD;
    }
    // println!("{:?}", hash);

    // ハッシュ値の計算
    let calc_hash = |l: usize, r: usize| {
        let val = hash[r] as isize - (hash[l - 1] * power100[r - l + 1] % MOD) as isize;
        if val < 0 {
            val + MOD as isize
        } else {
            val
        }
    };

    for (a, b, c, d) in abcd {
        if calc_hash(a, b) == calc_hash(c, d) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
