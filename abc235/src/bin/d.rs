use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    };

    // 操作
    let f1 = |x: usize| x * a;
    let f2 = |x: usize| {
        let last = x % 10;
        let mut pow = 0;
        let mut y = x;
        while y / 10 > 0 {
            y /= 10;
            pow += 1;
        }
        x / 10 + last * 10usize.pow(pow)
    };

    // BFS
    let mut count = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 0usize));

    while let Some((x, cnt)) = queue.pop_front() {
        if x == n {
            println!("{}", cnt);
            return;
        }

        let y = f1(x);
        if y <= 10usize.pow(6) && (!count.contains_key(&y) || cnt + 1 < *count.get(&y).unwrap()) {
            count.insert(y, cnt + 1);
            queue.push_back((y, cnt + 1));
        }
        if x >= 10 && x % 10 != 0 {
            let y = f2(x);
            if y <= 10usize.pow(6) && (!count.contains_key(&y) || cnt + 1 < *count.get(&y).unwrap())
            {
                count.insert(y, cnt + 1);
                queue.push_back((y, cnt + 1));
            }
        }
    }

    if !count.contains_key(&n) {
        println!("-1");
    } else {
        println!("{}", *count.get(&n).unwrap());
    }
}
