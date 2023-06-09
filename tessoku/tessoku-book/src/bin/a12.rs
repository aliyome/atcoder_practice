use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    // 答えを二分探索する
    let mut ng = 0;
    let mut ok = 10usize.pow(9);
    let total = |mid| {
        let mut sum = 0;
        for i in 0..n {
            sum += mid / a[i];
        }
        sum
    };
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if k <= total(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
