use proconio::input;

fn main() {
    input! {
      n: usize,
      l: usize,
      k: usize,
      mut a: [usize; n]
    }

    a.insert(0, 0);
    a.push(l);

    // スコアを二分探索で求める
    let is_ok = |len| {
        let mut count = 0;
        let mut left = 0;
        for right in 1..a.len() {
            if a[right] - a[left] >= len {
                count += 1;
                left = right;
            }
        }
        count >= k + 1
    };

    let mut ng = l;
    let mut ok = 0;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
