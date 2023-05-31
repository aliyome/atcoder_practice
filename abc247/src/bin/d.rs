use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut deq = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize
        }

        if t == 1 {
            input! {
                x: usize,
                c: usize
            }
            deq.push_back((x, c));
        } else {
            input! {
                mut c: usize
            }

            let mut ans = 0;
            while let Some((xx, cc)) = deq.pop_front() {
                if c <= cc {
                    ans += xx * c;
                    println!("{}", ans);
                    if c > 0 {
                        deq.push_front((xx, cc - c));
                    }
                    break;
                } else {
                    ans += xx * cc;
                    c -= cc;
                }
            }
        }
    }
}
