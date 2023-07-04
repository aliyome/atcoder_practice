use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
      n: usize,
      xy: [(isize, isize); n]
    }

    // 全探索: O(2^n) -> 2^100 -> TLE
    let mut q = VecDeque::new();
    q.push_back(xy[0]);
    let mut used = vec![false; n];

    let mut ans = vec![0];
    while let Some((x, y)) = q.pop_front() {
        let mut next = 0;
        let mut dist = std::isize::MAX;
        for j in 1..n {
            if used[j] {
                continue;
            }
            let (x2, y2) = xy[j];
            let d = (x2 - x).pow(2) + (y2 - y).pow(2);
            if d < dist {
                dist = d;
                next = j;
            }
        }

        if next == 0 {
            break;
        }

        used[next] = true;
        ans.push(next);
        q.push_back(xy[next]);
    }

    for &a in &ans {
        println!("{} ", a + 1);
    }

    println!("1");
}
