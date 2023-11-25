use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    if s[0] != t[0] || s[n - 1] != t[m - 1] {
        println!("No");
        return;
    }

    // 左から右へ
    let mut dp_ltr = vec![false; n];
    dp_ltr[0] = true;

    let mut pos = 0;
    while pos < n {
        if dp_ltr[pos] && s[i] == t[j] {
            dp_ltr[pos + 1] = true;
        }
    }

    // 右から左へのDP
    let mut dp_rtl = vec![false; n];
    for i in (0..=n - m).rev() {
        if (0..m).all(|j| s[i + j] == t[j]) {
            dp_rtl[i] = true;
        }
    }

    eprintln!("{:?}", dp_ltr);
    eprintln!("{:?}", dp_rtl);

    // 左から右へと右から左へのDPを組み合わせてチェック
    let mut possible = false;
    for i in 0..=n - m {
        if dp_ltr[i] && (i + m == n || dp_rtl[i + m]) {
            possible = true;
            break;
        }
    }

    println!("{}", if possible { "Yes" } else { "No" });
}
