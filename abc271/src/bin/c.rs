use std::collections::HashSet;
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    let mut set = HashSet::new();
    let mut left = 0;
    for a in a {
        if set.contains(&a) {
            left += 1;
            continue;
        }
        set.insert(a);
    }

    let mut list = set.iter().map(|x| *x).collect::<Vec<_>>();
    list.sort();

    let mut list = VecDeque::from(list);

    // ダブりを末尾に追加
    for _ in 0..left {
        list.push_back(0);
    }

    // println!("{:?}", list);

    for i in 1..=n {
        let front = list.front();
        if front.is_none() {
            println!("{}", i - 1);
            // println!("foo");
            return;
        }
        let front = front.unwrap();
        if i == *front {
            // println!("ok: {}", i);
            list.pop_front().unwrap();
            continue;
        }
        let back1 = list.pop_back();
        let back2 = list.pop_back();
        // println!("back1: {:?}, back2: {:?}", back1, back2);
        if back1.is_none() || back2.is_none() {
            println!("{}", i - 1);
            // println!("bar");
            return;
        }
        // let back1 = back1.unwrap();
        // let back2 = back2.unwrap();
        // if back1 <= i || back2 <= i {
        //     println!("{}", i - 1);
        //     println!("baz");
        //     return;
        // }
    }

    println!("{}", n);
}
