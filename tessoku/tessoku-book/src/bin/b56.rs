use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
      n: usize,
      q: usize,
      mut s: Chars,
      lr: [(usize, usize); q],
    }

    // char -> usize
    let mut t = s
        .clone()
        .into_iter()
        .map(|c| (c as u8 - b'a' + 1) as usize)
        .collect::<Vec<usize>>();
    let mut t_rev = t.clone();
    t_rev.reverse();

    // 1-indexed
    s.insert(0, ' ');
    t.insert(0, 0);
    t_rev.insert(0, 0);

    // B進数の事前計算 (B = 100)
    let mut pow100 = vec![0; n + 1];
    pow100[0] = 1;
    for i in 1..=n {
        pow100[i] = (pow100[i - 1] * 100) % MOD;
    }

    // ハッシュ値の事前計算
    // h[i] := s[1..=i] のハッシュ値
    let mut h = vec![0; n + 1];
    let mut h_rev = vec![0; n + 1];
    h[0] = 1;
    h_rev[0] = 1;
    for i in 1..=n {
        h[i] = (pow100[1] * h[i - 1] + t[i]) % MOD;
        h_rev[i] = (pow100[1] * h_rev[i - 1] + t_rev[i]) % MOD;
    }

    // ハッシュ値の計算
    let calc_hash = |l, r| {
        let mut res =
            h[r] as isize - (pow100[r - l + 1] as isize * h[l - 1] as isize % MOD as isize);
        if res < 0 {
            res += MOD as isize;
        }
        res
    };
    let calc_hash_rev = |ll, rr| {
        let l = n + 1 - rr;
        let r = n + 1 - ll;
        let mut res =
            h_rev[r] as isize - (pow100[r - l + 1] as isize * h_rev[l - 1] as isize % MOD as isize);
        if res < 0 {
            res += MOD as isize;
        }
        res
    };

    // クエリに答える
    for &(l, r) in &lr {
        let x = calc_hash(l, r);
        let y = calc_hash_rev(l, r);
        if x == y {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
