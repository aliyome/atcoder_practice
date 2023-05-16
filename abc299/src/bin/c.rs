use proconio::input;
use proconio::marker::Chars;

// WA
fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut ans = -1;
    let mut temp = 0;
    for i in 0..n {
        if s[i] == 'o' {
            temp += 1;
            if i < n - 1 && s[i + 1] == '-' {
                ans = ans.max(temp);
            }
        } else {
            if 0 < i && s[i - 1] == 'o' {
                ans = ans.max(temp);
            }
            temp = 0;
        }
    }

    println!("{}", ans);
}
