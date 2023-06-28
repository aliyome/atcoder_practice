use proconio::input;

fn main() {
    input! {
      n: usize,
      x: usize,
      y: usize,
      a:[usize; n]
    }

    // Grundy数
    let mut grundy = vec![std::usize::MAX; 100001];
    grundy[0] = 0;

    // i: 石の数
    for i in 1..=100000 {
        // 到達可能なgrundy数を集める
        let mut gg = vec![];
        if i >= x {
            gg.push(grundy[i - x]);
        }
        if i >= y {
            gg.push(grundy[i - y]);
        }

        // grundy数
        for k in 0..=100000 {
            if gg.contains(&k) {
                continue;
            } else {
                grundy[i] = k;
                break;
            }
        }
    }

    // 山iのgrundy数をXORする
    let mut ans = 0;
    for i in 0..n {
        ans ^= grundy[a[i]];
    }

    if ans != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
