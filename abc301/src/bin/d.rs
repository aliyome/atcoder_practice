use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        n: u64,
    }

    let mut rev_s = s.clone();
    rev_s.reverse();

    // base[i] = 左から数えて i 桁目に必ず足す必要がある数
    let mut base = vec![0; s.len()];
    for i in 0..s.len() {
        if i > 0 {
            base[i] = base[i - 1];
        }
        if rev_s[i] == '1' {
            base[i] += 1 << i;
        }
    }
    base.reverse();
    base.push(0);

    let mut ans = 0;
    for i in 0..s.len() {
        let keta = s.len() - i - 1;
        let can_1 = ans + (1 << keta) + base[i + 1] <= n;
        // println!("i:{} {} {} {}", i, keta, 1 << keta, can_1);
        // 必ずオーバーする
        if !can_1 && s[i] == '1' {
            println!("-1");
            return;
        }
        if s[i] == '0' {
            ans += 0;
        } else if s[i] == '1' {
            ans += 1 << keta;
        } else if s[i] == '?' && can_1 {
            ans += 1 << keta;
        }
    }

    println!("{}", ans);
}
