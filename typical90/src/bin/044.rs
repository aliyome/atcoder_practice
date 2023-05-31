use std::collections::VecDeque;

use proconio::input;

// Deque を使って愚直に実装するだけで AC が出てしまった
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize; n],
    }

    let mut a = VecDeque::from(a);

    for _ in 0..q {
        input! {
            t: usize,
            x: isize,
            y: isize
        }
        match t {
            1 => {
                let x = x - 1;
                let y = y - 1;
                let tmp = a[x as usize];
                a[x as usize] = a[y as usize];
                a[y as usize] = tmp;
            }
            2 => {
                let tmp = a.pop_back().unwrap();
                a.push_front(tmp);
            }
            3 => {
                println!("{}", a[x as usize - 1]);
            }
            _ => unreachable!(),
        }
    }
}
