use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n]
    };

    let mut ans = 0;
    let mut now = 0;
    for day in 0..d {
        let mut ok = true;
        for i in 0..n {
            if s[i][day] == 'x' {
                ok = false;
                break;
            }
        }
        if ok == true {
            now += 1;
        } else {
            now = 0;
        }
        ans = ans.max(now);
    }
    println!("{}", ans);
}
