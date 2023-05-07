use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    };
    let mut ans = 0;
    let mut stack: Vec<(usize, usize, HashSet<usize>)> = vec![];
    let mut his = HashSet::new();
    his.insert(a[0][0]);
    stack.push((0, 0, his));
    while let Some((x, y, history)) = stack.pop() {
        // println!("{} {} {:?}", x, y, history);
        if x == w - 1 && y == h - 1 {
            ans += 1;
            continue;
        }
        if x < w - 1 && !history.contains(&a[y][x + 1]) {
            let mut h = history.clone();
            h.insert(a[y][x + 1]);
            stack.push((x + 1, y, h));
        }
        if y < h - 1 && !history.contains(&a[y + 1][x]) {
            let mut h = history.clone();
            h.insert(a[y + 1][x]);
            stack.push((x, y + 1, h));
        }
    }

    println!("{}", ans);
}
