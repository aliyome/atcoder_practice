use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut a = s[i].clone();
            a.extend(s[j].clone());

            let mut is_ok = true;
            for k in 0..a.len() {
                if a[k] == a[a.len() - 1 - k] {
                    continue;
                } else {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
