use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 12]
    };

    let mut ans = 0;
    for i in 0..12 {
        for &c in &s[i] {
            if c == 'r' {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
