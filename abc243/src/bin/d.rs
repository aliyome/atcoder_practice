use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut x: usize,
        s: Chars
    };

    // 高速化
    let mut t = vec![];
    for &c in &s {
        match c {
            'U' => {
                if t.last() == Some(&'L') || t.last() == Some(&'R') {
                    t.pop();
                } else {
                    t.push(c);
                }
            }
            'L' | 'R' => t.push(c),
            _ => unreachable!(),
        }
    }

    // 愚直に計算
    for c in t {
        match c {
            'U' => x /= 2,
            'L' => x *= 2,
            'R' => x = x * 2 + 1,
            _ => unreachable!(),
        }
    }

    println!("{}", x);
}
