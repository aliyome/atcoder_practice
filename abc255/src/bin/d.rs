use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        x: [i64; q]
    };

    // // 全探索 O(NQ) -> TLE 10^10
    // for x in x {
    //     let mut ans = 0;
    //     for i in 0..n {
    //         let diff = (a[i] - x).abs();
    //         ans += diff;
    //     }
    //     println!("{}", ans);
    // }

    // 二分探索 AC
    // ソート O(NlogN)
    // 累積和 O(N)
    // 二分探索 O(QlogN)
    let mut a = a;
    a.sort();

    // 累積和
    let mut sums: Vec<i64> = vec![0; n];
    for i in 0..n {
        sums[i] = if i == 0 { a[i] } else { sums[i - 1] + a[i] };
    }

    let is_ok = |v, x| a[v as usize] <= x;
    for x in x {
        let mut ok: i64 = 0;
        let mut ng: i64 = n as i64;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(mid, x) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let under: i64 = sums[ok as usize];
        let over: i64 = sums[n - 1] - under;
        let diff_under = ((ok + 1) * x - under).abs();
        let diff_over = ((n as i64 - ok - 1) * x - over).abs();
        // println!(
        //     "x:{}, ok:{}, a[ok]:{}, under:{}, over:{}, diff_under:{}, diff_over:{}",
        //     x, ok, a[ok as usize], under, over, diff_under, diff_over
        // );
        println!("{}", diff_under + diff_over);
    }
}
