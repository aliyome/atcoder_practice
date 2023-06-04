use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        xy: [(i32, i32); n],
    }

    let mut queue = VecDeque::new();
    let mut ans = vec![false; n];
    ans[0] = true;
    queue.push_back(0);

    while let Some(v) = queue.pop_front() {
        for i in 0..n {
            if ans[i] {
                continue;
            }
            let x = xy[v].0 - xy[i].0;
            let y = xy[v].1 - xy[i].1;
            if x * x + y * y <= d * d {
                ans[i] = true;
                queue.push_back(i);
            }
        }
    }
    for i in 0..n {
        if ans[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
