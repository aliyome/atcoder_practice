use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        b: [i64; q],
    }

    // 二分探索するためにaをソートして番兵を追加する
    let mut a = a;
    a.sort();

    // 二分探索の結果を出力する
    for b in b {
        println!("{}", binary_search(&a, b));
    }
}

fn binary_search(a: &Vec<i64>, x: i64) -> i64 {
    let mut ok = 0;
    let mut ng = a.len() - 1;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if a[mid] <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let diff1 = (a[ok] - x).abs();
    let diff2 = (a[ng] - x).abs();
    if diff1 < diff2 {
        diff1
    } else {
        diff2
    }
}
