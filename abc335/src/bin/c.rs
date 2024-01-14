use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, // ドラゴンの部位の数
        q: usize, // クエリの数
    }

    // ドラゴンの各部位の位置を保持するキュー
    let mut parts: VecDeque<(i64, i64)> = (1..=n as i64).map(|i| (i, 0)).collect();

    for _ in 0..q {
        input! {
            query_type: i32, // クエリのタイプ
        }

        match query_type {
            // ドラゴンを移動するクエリ
            1 => {
                input! {
                    c: char, // 移動方向
                }

                // 先頭の部位（頭）を移動させる
                let (x, y) = *parts.front().unwrap();
                let new_head = match c {
                    'R' => (x + 1, y),
                    'L' => (x - 1, y),
                    'U' => (x, y + 1),
                    'D' => (x, y - 1),
                    _ => (x, y),
                };
                parts.push_front(new_head);
                parts.pop_back();
            }
            // 特定の部位の位置を出力するクエリ
            2 => {
                input! {
                    p: usize, // 部位の番号
                }
                let (x, y) = parts[p - 1];
                println!("{} {}", x, y);
            }
            _ => unreachable!(),
        }
    }
}
