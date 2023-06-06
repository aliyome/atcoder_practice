use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    };

    // 前から何文字目まで一致しているか
    let mut fronts = vec![true; t.len() + 1];
    for i in 0..t.len() {
        if !fronts[i] {
            fronts[i + 1] = false;
            continue;
        }
        if s[i] != t[i] && s[i] != '?' && t[i] != '?' {
            fronts[i + 1] = false;
        }
    }
    // 後ろ空何文字目まで一致しているか
    let mut back = vec![true; t.len() + 1];
    let mut s_rev = s.clone();
    let mut t_rev = t.clone();
    s_rev.reverse();
    t_rev.reverse();
    for i in 0..t.len() {
        if !back[i] {
            back[i + 1] = false;
            continue;
        }
        if s_rev[i] != t_rev[i] && s_rev[i] != '?' && t_rev[i] != '?' {
            back[i + 1] = false;
        }
    }

    for i in 0..=t.len() {
        if fronts[i] && back[t.len() - i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
