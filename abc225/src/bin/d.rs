use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    // 次・前の要素を保持する配列
    let mut next = vec![0; n + 1];
    let mut prev = vec![0; n + 1];
    for i in 0..=n {
        next[i] = i;
        prev[i] = i;
    }

    // クエリ処理
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: usize,
                    y: usize
                };

                next[x] = y;
                prev[y] = x;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize
                };

                next[x] = x;
                prev[y] = y;
            }
            3 => {
                input! {
                    x: usize
                };
                let mut deq = VecDeque::new();
                deq.push_back(x);

                // 前に辿る
                let mut cur = x;
                while cur != prev[cur] {
                    cur = prev[cur];
                    deq.push_front(cur);
                }

                // 後ろに辿る
                cur = x;
                while cur != next[cur] {
                    cur = next[cur];
                    deq.push_back(cur);
                }

                // 出力
                print!("{} ", deq.len());
                for i in deq {
                    print!("{} ", i);
                }
                println!();
            }
            _ => unreachable!(),
        }
    }
}
