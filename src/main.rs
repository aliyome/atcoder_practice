use proconio::input;
use std::vec::Vec;

fn solve(n: i32, l: i32, k: i32, a: &Vec<i32>) -> i32 {
    // 二分探索
    let mut ok: i32 = 0;
    let mut ng: i32 = l;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if cfg!(debug_assertions) {
            println!("ok: {}, ng: {}, mid: {}", ok, ng, mid);
        }
        // mid 以上の長さのピースに分割できるか
        if is_ok(mid, n, l, k, a) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn is_ok(score: i32, n: i32, l: i32, k: i32, a: &Vec<i32>) -> bool {
    let mut cnt = 0;
    let mut pre = 0;
    for i in 0..n {
        if cfg!(debug_assertions) {
            println!("score: {}, cnt: {}, pre: {}, i: {}", score, cnt, pre, i);
        }
        // score 以上の長さのピースに分割する
        if a[i as usize] - pre >= score {
            cnt += 1;
            pre = a[i as usize];
        }
    }
    // k 個以上に分割できるか
    if cnt > k || (cnt == k && l - pre >= score) {
        return true;
    }
    return false;
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [i32; n],
    }
    println!("{}", solve(n as i32, l as i32, k as i32, &a));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_solved() {
        assert_eq!(super::solve(3, 34, 1, &vec![8, 13, 26]), 13);
        assert_eq!(super::solve(7, 45, 2, &vec![7, 11, 16, 20, 28, 34, 38]), 12);
        assert_eq!(super::solve(3, 100, 1, &vec![28, 54, 81]), 46);
    }
}
